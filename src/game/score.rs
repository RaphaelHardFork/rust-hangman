use std::{cmp::min, path::Path};

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
    pub fn calculate_score(word: &str, value: usize) -> Result<Self> {
        let timestamp = format_time(now_utc())?;

        Ok(Self {
            value,
            word: word.to_string(),
            timestamp,
        })
    }

    pub fn load_score_from_json(file_path: &Path) -> Result<Vec<Score>> {
        let scores = load_from_json::<Vec<Score>>(file_path)?;
        Ok(scores)
    }
}

// region:			--- Pure functions

pub fn score_value(progress: usize, attemps: usize) -> usize {
    let full_letter = min(progress, 3);
    let mut small_letter = 0;

    if progress >= 3 {
        small_letter = progress - 3;
    }

    let value = full_letter * 100 + small_letter * 50;

    if value < attemps * 50 {
        0
    } else {
        value - attemps * 50
    }
}

// endregion:		--- Pure functions
