use crate::Result;
use serde::Serialize;

use crate::utils::time::{format_time, now_utc};

#[derive(Debug, Serialize)]
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
}
