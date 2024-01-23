use crate::dict::Dict;
use crate::hangman::Hangman;
use crate::utils::cli::info;
use crate::Result;

pub struct Game {
    pub dict: Dict,
    pub hangman: Hangman,
    pub word: String,
    pub guess: String,
}

impl Game {
    pub fn init_game() -> Result<Self> {
        let dict = Dict::new();
        let word = dict.get_random_word()?;
        let guess = _string_to_guess(&word, 0);

        println!(
            "{}",
            info(
                &"Welcome to Hangman game!\n\nThe goal is to guess the word in 5 attempt\n\n"
                    .to_string()
            )
        );

        Ok(Self {
            dict,
            hangman: Hangman::new(),
            word,
            guess,
        })
    }

    pub fn new_hangman(&mut self) -> Result<()> {
        let word = self.dict.get_random_word()?;
        self.guess = _string_to_guess(&word, 0);
        self.word = word;
        self.hangman = Hangman::new();

        Ok(())
    }

    pub fn quit(&self) {
        println!("{}", info(&"Bye! See you soon".to_string()));
    }

    pub fn update_guess(&mut self) {
        self.guess = _string_to_guess(&self.word, self.hangman.progress)
    }
}

fn _string_to_guess(word: &str, progress: usize) -> String {
    word.chars()
        .enumerate()
        .map(|(index, l)| {
            if index < progress {
                l.to_string()
            } else {
                '_'.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
