use console::{style, Style, StyledObject};
use dialoguer::{theme::ColorfulTheme, Input};

use crate::Result;

// region:			--- Prompt

pub fn prompt(text: &str) -> Result<String> {
    let theme = ColorfulTheme {
        prompt_style: Style::new().for_stderr().color256(56),
        prompt_suffix: style("?".to_string()).color256(56).for_stderr(),
        // prompt_prefix: style("?".to_string()).color256(56).for_stderr(),
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

pub fn letter_prompt(text: &str) -> Result<char> {
    let theme = ColorfulTheme {
        prompt_style: Style::new().for_stderr().color256(67),
        prompt_suffix: style("?".to_string()).color256(67).for_stderr(),
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

// endregion:		--- Prompt

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
