use console::{style, Style, StyledObject};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use lazy_regex::regex_is_match;

use crate::{game::Player, Result};

// 8-bit color picker => https://codepen.io/kevinli/pen/GRpXOvo

// region:			--- Text Prompt

pub fn closed_prompt(text: &str) -> Result<String> {
    let theme = ColorfulTheme {
        prompt_style: Style::new().for_stderr().black(),
        prompt_suffix: style("?".to_string()).black().for_stderr(),
        ..Default::default()
    };

    let input = Input::with_theme(&theme).validate_with(|input: &String| -> Result<()> {
        match input.as_str() {
            "y" | "n" => Ok(()),
            _ => Err("Only 'y' or 'n' accepted".into()),
        }
    });

    let res = input.with_prompt(text).interact_text()?;

    Ok(res)
}

pub fn username_prompt(text: &str) -> Result<String> {
    let theme = ColorfulTheme {
        prompt_style: Style::new().for_stderr().black(),
        prompt_suffix: style("?".to_string()).black().for_stderr(),
        ..Default::default()
    };

    let input = Input::with_theme(&theme).validate_with(|input: &String| -> Result<()> {
        if regex_is_match!(r"^[a-zA-Z0-9]+$", input) {
            Ok(())
        } else {
            Err("Invalid character(s)".into())
        }
    });

    let res = input.with_prompt(text).interact_text()?;

    Ok(res)
}

pub fn letter_prompt(text: &str) -> Result<char> {
    let theme = ColorfulTheme {
        prompt_style: Style::new().for_stderr().blue(),
        prompt_suffix: style("?".to_string()).blue().for_stderr(),
        ..Default::default()
    };

    let input = Input::with_theme(&theme).validate_with(|input: &String| -> Result<()> {
        if input.len() > 1 {
            Err("Only one letter accepted".into())
        } else {
            Ok(())
        }
    });

    let res = input
        .with_prompt(text)
        .interact_text()?
        .to_uppercase()
        .chars()
        .next()
        .ok_or("Unable to convert to char")?;

    Ok(res)
}

// endregion:		--- Text Prompt

// region:			--- Select

pub fn select_user(usernames: &Vec<String>, text: &str) -> Result<Option<String>> {
    let theme = ColorfulTheme {
        prompt_style: Style::new().blue(),
        active_item_prefix: style("๏".to_string()).color256(178),
        active_item_style: Style::new().color256(178).bold(),

        ..Default::default()
    };

    let selection = Select::with_theme(&theme)
        .with_prompt(text)
        .items(&usernames)
        .item("Create an user")
        .interact()?;

    if selection == usernames.len() {
        Ok(None)
    } else {
        let username = usernames
            .get(selection)
            .ok_or("Username selected not found")?;
        Ok(Some(username.to_owned()))
    }
}

// endregion:		--- Select

// region:			--- Texts
pub fn info(text: &String) -> StyledObject<String> {
    style(text.to_owned()).color256(25).bold()
}

pub fn loose(text: &String) -> StyledObject<String> {
    style(text.to_owned()).color256(1)
}

pub fn loose_b(text: &String) -> StyledObject<String> {
    style(text.to_owned()).color256(1).bold()
}

pub fn win_b(text: &String) -> StyledObject<String> {
    style(text.to_owned()).color256(43).bold()
}

pub fn win(text: &String) -> StyledObject<String> {
    style(text.to_owned()).color256(43)
}

pub fn hangman(text: &String) -> StyledObject<String> {
    style(text.to_owned()).color256(1)
}
// endregion:		--- Texts
