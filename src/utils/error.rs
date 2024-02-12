use derive_more::From;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, From, Serialize)]
pub enum Error {
    // Files
    CreateFileFailed(String),
    JsonSerialisationFail,
    // Modules
    #[from]
    File(#[serde_as(as = "DisplayFromStr")] std::io::Error),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
