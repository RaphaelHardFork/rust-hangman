mod dict;
mod hangman;

use self::dict::Dict;
use self::hangman::Hangman;
use crate::utils::cli::{info, letter_prompt, loose, loose_b, prompt, win, win_b};
use crate::utils::string_to_guess;
use crate::Result;

pub struct Game {
    pub dict: Dict,
    pub hangman: Hangman,
    pub word: String,
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
        println!("{}", info(&"\nBye! See you soon\n".to_string()));
    }
}

// endregion:		--- Game Logic
