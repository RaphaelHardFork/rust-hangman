mod dict;
mod hangman;
mod player;
mod score;

pub use self::player::Player;

use self::dict::Dict;
use self::hangman::Hangman;
use self::score::Score;
use crate::utils::cli::{info, letter_prompt, loose, loose_b, prompt, win, win_b};
use crate::utils::string_to_guess;
use crate::Result;

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
                &"Welcome to Hangman game!\n\nThe goal is to guess the word in 5 attempt\n\n"
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

// region:			--- Game Logic

impl Game {
    pub fn display_turn(&self) {
        self.hangman.print_hangman();
        println!("Attempts {}/5\n\n", self.hangman.attemps);
        println!(
            "Word to guess: {}\n",
            string_to_guess(&self.word, self.hangman.progress)
        );
    }

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

    pub fn is_game_over(&self) -> bool {
        // game winned
        if self.hangman.progress == self.word.len() {
            println!(
                "{} {}\n",
                win(&"Congratulations! You guessed the word: ".to_string()),
                win_b(&self.word)
            );
            true
        } else if self.hangman.attemps == 5 {
            println!(
                "{} {}\n",
                loose(&"You lose the word was: ".to_string()),
                loose_b(&self.word)
            );
            true
        } else {
            false
        }
    }

    pub fn calculate_score(&self) -> Result<Score> {
        Score::calculate_score(&self.word, self.hangman.attemps)
    }

    pub fn will_play_again(&mut self) -> Result<bool> {
        let input = prompt("Play again (y/n)")?;
        if input.as_str() == "y" {
            self.new_hangman()?;
            Ok(true)
        } else {
            self.quit();
            Ok(false)
        }
    }

    pub fn quit(&self) {
        // should save score
        println!("{}", info(&"\nBye! See you soon\n".to_string()));
    }
}

// endregion:		--- Game Logic

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
