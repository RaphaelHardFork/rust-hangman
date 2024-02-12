use globset::{Glob, GlobSet, GlobSetBuilder};
use walkdir::WalkDir;

use crate::Result_legacy;
use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use super::{Error, Result};

// region:			--- File write/parse
pub fn save_to_json<T>(file: impl AsRef<Path>, data: &T) -> Result<()>
where
    T: serde::Serialize,
{
    let file = file.as_ref();
    let file = File::create(file)?;

    serde_json::to_writer_pretty(file, data).map_err(|_| Err(Error::JsonSerialisationFail))?;
    Ok(())
}

pub fn load_from_json<T>(file: impl AsRef<Path>) -> Result_legacy<T>
where
    T: serde::de::DeserializeOwned,
{
    let val = serde_json::from_reader(get_reader(file.as_ref())?)?;
    Ok(val)
}

pub fn load_from_txt(file: impl AsRef<Path>) -> Result_legacy<Vec<String>> {
    let reader = get_reader(file.as_ref())?;
    let mut words: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        words.push(line.to_uppercase());
    }

    Ok(words)
}
// endregion:		--- File write/parse

// region:			--- File utils
fn get_reader(file: &Path) -> Result_legacy<BufReader<File>> {
    let Ok(file) = File::open(file) else {
        return Err(format!("File not found: {}", file.display()).into());
    };

    Ok(BufReader::new(file))
}
// endregion:		--- File utils

// region:			--- Dir utils
/// Returns true if  one or more dir was created
pub fn ensure_dir(dir: &Path) -> Result_legacy<bool> {
    if dir.is_dir() {
        Ok(false)
    } else {
        fs::create_dir_all(dir)?;
        Ok(true)
    }
}

pub fn list_files(
    dir: &Path,
    include_globs: Option<&[&str]>,
    exclude_globs: Option<&[&str]>,
) -> Result_legacy<Vec<PathBuf>> {
    let base_dir_exclude = base_dir_exclude_globs()?;

    // recursive depth
    let depth = include_globs
        .map(|globs| globs.iter().any(|&g| g.contains("**")))
        .map(|v| if v { 100 } else { 1 })
        .unwrap_or(1);

    // prep globs
    let include_globs = include_globs.map(get_glob_set).transpose()?;
    let exclude_globs = exclude_globs.map(get_glob_set).transpose()?;

    // build file iterator
    let walk_dir_it = WalkDir::new(dir)
        .max_depth(depth)
        .into_iter()
        .filter_entry(|e| {
            // if dir, check if it's excluded
            if e.file_type().is_dir() {
                !base_dir_exclude.is_match(e.path())
            }
            // if file, apply the globs
            else {
                // is the file is excluded?
                if let Some(exclude_globs) = exclude_globs.as_ref() {
                    if exclude_globs.is_match(e.path()) {
                        return false;
                    }
                }
                match include_globs.as_ref() {
                    // add included globs
                    Some(globs) => globs.is_match(e.path()),
                    // to the already listed files
                    None => true,
                }
            }
        })
        .filter_map(|e| e.ok().filter(|e| e.file_type().is_file()));

    let paths = walk_dir_it.map(|e| e.into_path());

    Ok(paths.collect())
}

fn base_dir_exclude_globs() -> Result_legacy<GlobSet> {
    get_glob_set(&[".git", "target"])
}

pub fn get_glob_set(globs: &[&str]) -> Result_legacy<GlobSet> {
    let mut builder = GlobSetBuilder::new();
    for glob in globs {
        builder.add(Glob::new(glob)?);
    }
    Ok(builder.build()?)
}
// endregion:		--- Dir utils
