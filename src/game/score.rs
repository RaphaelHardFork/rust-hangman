use std::{cmp::min, path::Path};

use crate::{utils::files::load_from_json, Result_legacy};
use serde::{Deserialize, Serialize};

use crate::utils::time::{format_time, now_utc};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Score {
    pub value: usize,
    pub word: String,
    pub timestamp: String,
}

impl Score {
    pub fn calculate_score(word: &str, value: usize) -> Result_legacy<Self> {
        let timestamp = format_time(now_utc())?;

        Ok(Self {
            value,
            word: word.to_string(),
            timestamp,
        })
    }

    pub fn load_score_from_json(file_path: &Path) -> Result_legacy<Vec<Score>> {
        let scores = load_from_json::<Vec<Score>>(file_path)?;
        Ok(scores)
    }
}

// region:			--- Pure functions

pub fn score_value(progress: usize, attemps: usize, hint_used: bool) -> usize {
    let full_letter = min(progress, 3);
    let mut small_letter = 0;

    if progress >= 3 {
        small_letter = progress - 3;
    }

    let value = full_letter * 100 + small_letter * 50;

    if value < attemps * 50 {
        0
    } else {
        let res = value - attemps * 50;
        if hint_used {
            res / 2
        } else {
            res
        }
    }
}

// endregion:		--- Pure functions
