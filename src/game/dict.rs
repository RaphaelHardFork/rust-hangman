use std::path::Path;

use rand::Rng;

use crate::{utils::files::load_from_txt, Result};

const DICT_DIR: &str = "dictionaries";

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Dict {
    pub words: Vec<String>,
}

impl Dict {
    pub fn load_or_create() -> Result<Self> {
        let dict = Dict::load_dict();

        match dict {
            Ok(words) => Ok(Self { words }),
            Err(_) => {
                let words: Vec<String> = WORDS
                    .iter()
                    .map(|&w| w.to_string().to_uppercase())
                    .collect();
                println!("Failed to load a dictionary, fallback on a small one.");
                Ok(Self { words })
            }
        }
    }

    pub fn get_random_word(&self) -> Result<String> {
        let rand_nb = rand::thread_rng().gen_range(0..self.words.len());

        Dict::load_dict()?;

        if let Some(word) = self.words.get(rand_nb) {
            Ok(word.to_string())
        } else {
            Err("Word never find".into())
        }
    }

    pub fn load_dict() -> Result<Vec<String>> {
        let file_path: &Path = DICT_DIR.as_ref();
        let file_path = file_path.join("en.txt");

        load_from_txt(file_path)
    }
}

const WORDS: [&str; 37] = [
    "word", "time", "number", "way", "people", "water", "day", "part", "sound", "work", "place",
    "year", "back", "thing", "name", "sentence", "man", "line", "boy", "farm", "end", "men",
    "land", "home", "hand", "picture", "air", "animal", "house", "page", "letter", "point",
    "mother", "answer", "America", "world", "food",
];
