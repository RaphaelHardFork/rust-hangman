mod error;
mod game;
mod utils;

pub use self::error::{Error, Result};

use crate::game::Game;
use crate::utils::cli::select_user;
use console::Term;
use utils::cli::username_prompt;

fn main() -> Result<()> {
    let mut game = Game::init_game()?;

    let usernames = game.get_usernames()?;
    let username = select_user(&usernames, "Choose an user")?;

    if let Some(username) = username {
        game.register_user(&username);
        game.load_scores(&username)?;
    } else {
        let username = username_prompt("Choose an username")?;
        game.register_user(&username);
    }

    let term = Term::stdout();

    loop {
        // clean the screen
        term.clear_screen()?;
        println!("New game with: {}", game.word); // DEV for answer

        // display game state
        game.print_round_start();

        // is the game is over
        if game.is_game_over()? {
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
