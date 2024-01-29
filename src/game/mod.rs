mod dict;
mod hangman;
mod player;
mod score;

pub use self::player::Player;

use self::dict::Dict;
use self::hangman::Hangman;
use self::player::SCORES_DIR;
use self::score::Score;
use crate::game::score::score_value;
use crate::utils::cli::{closed_prompt, info, letter_prompt, loose, loose_b, win, win_b};
use crate::utils::files::list_files;
use crate::utils::string_to_guess;
use crate::Result;
use lazy_regex::regex_captures;
use std::collections::HashMap;
use std::path::PathBuf;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Game {
    pub dict: Dict,
    pub hangman: Hangman,
    pub word: String,
    pub user: Option<Player>,
}

// region:			--- Game Constructors
impl Game {
    pub fn init_game() -> Result<Self> {
        // load a dictionnary
        let dict = Dict::new();
        let word = dict.get_random_word()?;

        println!(
            "{}",
            info(
                &"Welcome to Hangman game!\n\nThe goal is to guess the word in 5 attempts, create an user to play.\n\n"
                    .to_string()
            )
        );

        Ok(Self {
            dict,
            word,
            hangman: Hangman::new(),
            user: None,
        })
    }

    pub fn new_hangman(&mut self) -> Result<()> {
        self.word = self.dict.get_random_word()?;
        self.hangman = Hangman::new();

        Ok(())
    }
}
// endregion:		--- Game Constructors

// region:			--- Game Messages

impl Game {
    pub fn print_round_start(&self) {
        self.hangman.print_hangman();
        println!("Attempts {}/5", self.hangman.attemps);
        println!(
            "Score: {}\n\n",
            score_value(self.hangman.progress, self.hangman.attemps)
        );
        println!(
            "Word to guess: {}\n",
            string_to_guess(&self.word, self.hangman.progress)
        );
    }

    pub fn print_round_win(&self) {
        println!(
            "{} {}\n",
            win(&"Congratulations! You guessed the word: ".to_string()),
            win_b(&self.word)
        );
    }

    pub fn print_round_lose(&self) {
        println!(
            "{} {}\n",
            loose(&"You lose the word was: ".to_string()),
            loose_b(&self.word)
        );
    }

    pub fn print_game_out(&self) {
        println!("{}", info(&"\nBye! See you soon\n".to_string()));
    }
}

// endregion:		--- Game Messages

// region:			--- Game Logic

impl Game {
    pub fn guess_a_letter(&mut self) -> Result<()> {
        let letter = letter_prompt("Guess the next letter")?;

        let letter_to_guess = self
            .word
            .chars()
            .nth(self.hangman.progress)
            .ok_or("No more letter to guess")?;

        if letter == letter_to_guess {
            self.hangman.progress();
        } else {
            self.hangman.attemp();
        }

        Ok(())
    }

    pub fn register_user(&mut self, username: &str) {
        self.user = Some(Player::create(username))
    }

    pub fn is_game_over(&mut self) -> Result<bool> {
        // game winned
        if self.hangman.progress == self.word.len() {
            self.print_round_win();
            self.calculate_score()?;
            Ok(true)
        } else if self.hangman.attemps == 5 {
            self.print_round_lose();
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn calculate_score(&mut self) -> Result<()> {
        if let Some(user) = &mut self.user {
            let value = score_value(self.hangman.progress, self.hangman.attemps);
            let score = Score::calculate_score(&self.word, value)?;
            user.scores.push(score);
        }
        Ok(())
    }

    pub fn will_play_again(&mut self) -> Result<bool> {
        let input = closed_prompt("Play again (y/n)")?;
        if input.as_str() == "y" {
            self.new_hangman()?;
            Ok(true)
        } else {
            self.quit()?;
            Ok(false)
        }
    }

    pub fn quit(&self) -> Result<()> {
        match &self.user {
            Some(user) => user.save_user()?,
            None => {}
        };
        self.print_game_out();
        Ok(())
    }
}

// endregion:		--- Game Logic

// region:			--- Game Players
impl Game {
    pub fn get_players_hashmap(&self) -> Result<HashMap<String, PathBuf>> {
        // read JSON
        let files = list_files(SCORES_DIR.as_ref(), None, None)?;

        // create the hashmap
        let players_hashmap: HashMap<String, PathBuf> = files
            .iter()
            .filter_map(|f| {
                let file_str = f.to_str()?;
                regex_captures!(r"^([^/]+)/([^/]+)\.json$", file_str)
                    .map(|(_, _, username)| (username.to_string(), f.to_owned()))
            })
            .collect();

        Ok(players_hashmap)
    }

    pub fn get_usernames(&self) -> Result<Vec<String>> {
        let players_hashmap = self.get_players_hashmap()?;
        let usernames: Vec<String> = players_hashmap.keys().map(|k| k.to_owned()).collect();

        Ok(usernames)
    }

    pub fn get_username_path(
        &self,
        players_hashmap: &mut HashMap<String, PathBuf>,
        username: &str,
    ) -> Result<PathBuf> {
        let path = players_hashmap
            .remove(username)
            .ok_or(format!("Cannot find path for {}", username))?;

        Ok(path)
    }

    pub fn load_scores(&mut self, username: &str) -> Result<()> {
        let mut players_hashmap = self.get_players_hashmap()?;

        let file_path = self.get_username_path(&mut players_hashmap, username)?;
        let scores = Score::load_score_from_json(file_path.as_path())?;

        if let Some(user) = &mut self.user {
            user.scores = scores;
        }

        Ok(())
    }
}
// endregion:		--- Game Players

// region:    --- Tests
#[cfg(test)]
mod tests {
    use super::*;

    pub type Result<T> = core::result::Result<T, Error>;
    pub type Error = Box<dyn std::error::Error>;

    #[test]
    fn test_init_game_ok() -> Result<()> {
        let game = Game::init_game()?;
        let fx_game = Game {
            dict: Dict::new(),
            hangman: Hangman::new(),
            word: game.word.clone(),
            user: None,
        };

        assert_eq!(game, fx_game);
        Ok(())
    }

    #[test]
    fn test_new_hangman_ok() -> Result<()> {
        let fx_hangman = Hangman {
            attemps: 0,
            progress: 0,
        };
        let mut game = Game::init_game()?;
        let word = game.word.clone();
        game.hangman.attemp();
        game.hangman.progress();

        game.new_hangman()?;

        assert_eq!(game.hangman, fx_hangman);
        assert_ne!(word, game.word);

        Ok(())
    }
}
// endregion:		--- Tests
