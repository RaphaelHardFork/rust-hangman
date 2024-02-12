use super::{Error, Result};
use crate::utils::files::load_from_txt;
use rand::Rng;
use std::path::Path;

const DICT_DIR: &str = "dictionaries";
const FALLBACK_WORDS: [&str; 37] = [
    "word", "time", "number", "way", "people", "water", "day", "part", "sound", "work", "place",
    "year", "back", "thing", "name", "sentence", "man", "line", "boy", "farm", "end", "men",
    "land", "home", "hand", "picture", "air", "animal", "house", "page", "letter", "point",
    "mother", "answer", "America", "world", "food",
];

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Dict {
    pub words: Vec<String>,
}

impl Dict {
    pub fn load_or_create() -> Self {
        let file_path: &Path = DICT_DIR.as_ref();
        let file_path = file_path.join("en.txt");

        if let Ok(words) = load_from_txt(file_path) {
            Self { words }
        } else {
            let words: Vec<String> = FALLBACK_WORDS
                .iter()
                .map(|&w| w.to_string().to_uppercase())
                .collect();
            println!("Failed to load a dictionary, fallback on a small one.");
            Self { words }
        }
    }

    pub fn get_random_word(&self) -> Result<String> {
        if self.words.is_empty() {
            Err(Error::EmptyDict)
        } else {
            let rand_nb = rand::thread_rng().gen_range(0..self.words.len());
            let word = &self.words[rand_nb];
            Ok(word.to_owned())
        }
    }
}
