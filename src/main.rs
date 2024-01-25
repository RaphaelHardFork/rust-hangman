mod error;
mod game;
mod utils;

use dialoguer::theme::ColorfulTheme;
use dialoguer::{MultiSelect, Select};
use std::path::Path;

pub use self::error::{Error, Result};

use crate::game::Game;
use crate::utils::cli::{closed_prompt, select_user};
use console::Term;
use game::Player;
use lazy_regex::regex_captures;
use utils::cli::username_prompt;
use utils::files::list_files;

fn main() -> Result<()> {
    let mut game = Game::init_game()?;

    let mut usernames = Player::load_users_from_json()?;
    let username = select_user(&usernames, "Choose an user")?;

    if let Some(username) = username {
        game.register_user(&username);
        game.load_scores(&username)?;
    } else {
        let username = username_prompt("Choose an username")?;
        game.user = Some(Player::create(&username));
    }

    let mut files = list_files(".scores".as_ref(), None, None)?;
    println!("->> files: {:?}", files);

    let files: Vec<String> = files
        .iter_mut()
        .map(|f| {
            let (_, _, username) =
                regex_captures!(r"^([^/]+)/([^/]+)\.json$", f.to_str().unwrap()).unwrap();
            username.to_string()
        })
        .collect();

    println!("->> files: {:?}", files);

    // continue? not very useful
    let input = closed_prompt("Are you ready (y/n)")?;
    if input.as_str() == "n" {
        game.quit();
        return Ok(());
    }

    let input = closed_prompt("Register as an user (y/n)")?;
    if input.as_str() == "y" {
        let username = username_prompt("Choose an username")?;
        game.register_user(&username);
    }

    let term = Term::stdout();

    loop {
        // clean the screen
        term.clear_screen()?;
        println!("New game with: {}", game.word); // DEV for answer

        // display game state
        game.display_turn();

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

fn for_test() -> Result<()> {
    let user = Player {
        username: "franck".to_string(),
        scores: vec![],
    };
    user.save_user()?;
    println!("->> User saved");
    Ok(())
}
