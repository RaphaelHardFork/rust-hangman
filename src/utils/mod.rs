pub mod cli;

pub fn string_to_guess(word: &str, progress: usize) -> String {
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

// region:    --- Tests
#[cfg(test)]
mod tests {
    use super::*;

    pub type Result<T> = core::result::Result<T, Error>;
    pub type Error = Box<dyn std::error::Error>;

    #[test]
    fn test_convert_string_to_guess_ok() -> Result<()> {
        let progress = 3;
        let fx_str = String::from("AMERICA");
        let fx_res = String::from("A M E _ _ _ _");

        let res = string_to_guess(&fx_str, progress);

        assert_eq!(res, fx_res);
        Ok(())
    }
}
// endregion:		--- Tests
