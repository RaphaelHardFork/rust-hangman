use std::path::Path;

use serde::Serialize;

use super::score::Score;
use crate::{
    utils::files::{ensure_dir, save_to_json},
    Result,
};

const DEFAULT_DIR: &str = ".scores";
const JSON_EXT: &str = ".json";

#[derive(Serialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Player {
    pub username: String,
    pub scores: Vec<Score>,
}

// region:			--- Constructors
impl Player {
    pub fn create(username: &str) -> Self {
        Self {
            username: username.to_string(),
            scores: vec![],
        }
    }
}
// endregion:		--- Constructors

impl Player {
    pub fn save_user(&self) -> Result<()> {
        let file: &Path = DEFAULT_DIR.as_ref();
        ensure_dir(file)?;
        let file_ext = self.username.clone() + JSON_EXT;
        save_to_json(file.join(file_ext), &self.scores)?;
        Ok(())
    }

    pub fn add_score(&mut self, score: Score) {
        self.scores.push(score)
    }
}
