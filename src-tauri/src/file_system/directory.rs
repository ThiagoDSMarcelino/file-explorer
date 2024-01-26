use std::fs::read_dir;
use std::path::{Path, PathBuf};

use crate::error::Error;
use crate::prelude::*;

#[tauri::command]
pub fn get_data_from_dir(path: &Path) -> Result<Vec<PathBuf>> {
    if !path.is_dir() {
        return Err(Error::Generic(format!(
            "Path is not a directory: {}",
            path.display()
        )));
    }

    Ok(read_dir(path)?
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .collect::<Vec<PathBuf>>())
}
