use std::path::Path;

use crate::{utils::files::load_from_json, Result};
use serde::{Deserialize, Serialize};

use crate::utils::time::{format_time, now_utc};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Score {
    value: usize,
    word: String,
    timestamp: String,
}

impl Score {
    pub fn calculate_score(word: &str, attemps: usize) -> Result<Self> {
        let timestamp = format_time(now_utc())?;
        let value = 300 + ((word.len() - 3) * 50) - attemps * 50;

        Ok(Self {
            value,
            word: word.to_string(),
            timestamp,
        })
    }

    pub fn calculate_pure_score(word: &str, attemps: usize) -> usize {
        let value = 300 + ((word.len() - 3) * 50) - attemps * 50;
        value
    }

    pub fn load_score_from_json(file_path: &Path) -> Result<Vec<Score>> {
        let scores = load_from_json::<Vec<Score>>(file_path)?;
        Ok(scores)
    }
}
