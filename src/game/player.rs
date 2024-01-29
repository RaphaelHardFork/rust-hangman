use super::score::Score;
use crate::utils::cli::info;
use crate::utils::files::{ensure_dir, list_files, save_to_json};
use crate::Result;
use lazy_regex::regex_captures;
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
            username: username.to_string(),
            scores: vec![],
        }
    }

    pub fn load_users_from_json() -> Result<Vec<String>> {
        let files = list_files(SCORES_DIR.as_ref(), None, None)?;

        let usernames: Vec<String> = files
            .iter()
            .filter_map(|f| {
                let file_str = f.to_str()?;
                regex_captures!(r"^([^/]+)/([^/]+)\.json$", file_str)
                    .map(|(_, _, username)| username.to_string())
            })
            .collect();

        Ok(usernames)
    }
}
// endregion:		--- Constructors

impl Player {
    pub fn save_user(&self) -> Result<()> {
        let file: &Path = SCORES_DIR.as_ref();
        ensure_dir(file)?;
        let file_ext = self.username.clone() + JSON_EXT;
        save_to_json(file.join(file_ext), &self.scores)?;
        println!("{}", info(&"User scores saved".to_string()));
        Ok(())
    }

    pub fn add_score(&mut self, score: Score) {
        self.scores.push(score)
    }
}
