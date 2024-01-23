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
