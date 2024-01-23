mod dict;
mod error;
mod game;
mod hangman;
mod utils;

pub use self::error::{Error, Result};

use crate::game::Game;
use crate::utils::cli::{letter_prompt, loose, loose_b, prompt, win, win_b};
use console::Term;

fn main() -> Result<()> {
    let mut game = Game::init_game()?;

    let input = prompt("Are you ready (y/n)")?;
    if input.as_str() == "n" {
        game.quit();
        return Ok(());
    }

    let term = Term::stdout();

    loop {
        term.clear_screen()?;
        println!("New game with: {}\n{}", game.word, game.guess); // DEV

        game.hangman.print_hangman();
        println!("Attempts {}/5\n\n", game.hangman.attemps);
        println!("Word to guess: {}\n", game.guess);

        // Game over
        if game.hangman.attemps == 5 {
            println!(
                "{} {}\n",
                loose(&"You lose the word was: ".to_string()),
                loose_b(&game.word)
            );
            let input = prompt("Play again (y/n)")?;
            match input.as_str() {
                "y" => {
                    game.new_hangman()?;
                    continue;
                }
                "n" => {
                    game.quit();
                    break;
                }
                _ => {}
            }
        }

        let letter = letter_prompt("Guess the next letter")?;

        // validate letter
        let letter_to_guess = game
            .word
            .chars()
            .nth(game.hangman.progress)
            .ok_or("No more letter to guess")?;

        if letter == letter_to_guess {
            game.hangman.progress();
            game.update_guess();
        } else {
            game.hangman.attemp();
        }

        // game winned
        if game.hangman.progress == game.word.len() {
            println!(
                "{} {}\n",
                win(&"Congratulations! You guessed the word: ".to_string()),
                win_b(&game.word)
            );
            let input = prompt("Play again (y/n)")?;
            match input.as_str() {
                "y" => {
                    game.new_hangman()?;
                    continue;
                }
                "n" => {
                    game.quit();
                    break;
                }
                _ => {}
            }
        }
    }

    Ok(())
}
