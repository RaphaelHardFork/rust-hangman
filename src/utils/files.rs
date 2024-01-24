use crate::Result;
use std::{
    fs::{self, File},
    io::BufReader,
    path::Path,
};

// region:			--- File write/parse
pub fn save_to_json<T>(file: impl AsRef<Path>, data: &T) -> Result<()>
where
    T: serde::Serialize,
{
    let file = file.as_ref();
    let file = File::create(file).map_err(|e| format!("Cannot create file '{:?}': {}", file, e))?;

    serde_json::to_writer_pretty(file, data)?;
    Ok(())
}

pub fn load_from_json<T>(file: impl AsRef<Path>) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let val = serde_json::from_reader(get_reader(file.as_ref())?)?;
    Ok(val)
}
// endregion:		--- File write/parse

// region:			--- File utils
fn get_reader(file: &Path) -> Result<BufReader<File>> {
    let Ok(file) = File::open(file) else {
        return Err(format!("File not found: {}", file.display()).into());
    };

    Ok(BufReader::new(file))
}
// endregion:		--- File utils

// region:			--- Dir utils
/// Returns true if  one or more dir was created
pub fn ensure_dir(dir: &Path) -> Result<bool> {
    if dir.is_dir() {
        Ok(false)
    } else {
        fs::create_dir_all(dir)?;
        Ok(true)
    }
}
// endregion:		--- Dir utils
