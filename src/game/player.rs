use super::score::Score;
use crate::utils::cli::info;
use crate::utils::files::{ensure_dir, save_to_json};
use crate::Result;
use serde::Serialize;
use std::path::Path;

pub const SCORES_DIR: &str = ".scores";
const JSON_EXT: &str = ".json";

#[derive(Serialize, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Player {
    pub username: String,
    pub scores: Vec<Score>,
}

// region:			--- Constructors
impl Player {
    pub fn create(username: &str) -> Self {
        Self {
            username: username.to_string().to_lowercase(),
            scores: vec![],
        }
    }

    pub fn save_user(&self) -> Result<()> {
        let file: &Path = SCORES_DIR.as_ref();
        ensure_dir(file)?;
        let file_ext = self.username.clone() + JSON_EXT;
        save_to_json(file.join(file_ext), &self.scores)?;
        println!("{}", info(&"User scores saved".to_string()));
        Ok(())
    }
}
// endregion:		--- Constructors
