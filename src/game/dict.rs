use rand::Rng;

use crate::Result;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Dict {
    pub words: Vec<String>,
}

impl Dict {
    pub fn new() -> Self {
        let words: Vec<String> = WORDS
            .iter()
            .map(|&w| w.to_string().to_uppercase())
            .collect();

        Self { words }
    }

    pub fn get_random_word(&self) -> Result<String> {
        let rand_nb = rand::thread_rng().gen_range(0..self.words.len());

        if let Some(word) = self.words.get(rand_nb) {
            Ok(word.to_string())
        } else {
            Err("Word never find".into())
        }
    }
}

const WORDS: [&str; 37] = [
    "word", "time", "number", "way", "people", "water", "day", "part", "sound", "work", "place",
    "year", "back", "thing", "name", "sentence", "man", "line", "boy", "farm", "end", "men",
    "land", "home", "hand", "picture", "air", "animal", "house", "page", "letter", "point",
    "mother", "answer", "America", "world", "food",
];
