mod error;
mod game;
mod utils;

pub use self::error::{Error, Result};

use crate::game::Game;
use crate::utils::cli::prompt;
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
        // clean the screen
        term.clear_screen()?;
        println!("New game with: {}", game.word); // DEV for answer

        // display game state
        game.display_turn();

        // is the game is over
        if game.is_game_over() {
            if game.will_play_again()? {
                continue;
            } else {
                break;
            }
        }

        // if the game continue guess another letter
        game.guess_a_letter()?;
    }

    Ok(())
}
