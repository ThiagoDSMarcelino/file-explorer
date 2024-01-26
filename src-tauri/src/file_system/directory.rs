use std::fs::read_dir;
use std::path::{Path, PathBuf};

use crate::error::Error;
use crate::prelude::*;

/// Retrieves the data from a directory specified by the given path.
///
/// # Arguments
///
/// * `path` - The path to the directory.
///
/// # Returns
///
/// Returns a `Result` containing a vector of `PathBuf` objects representing the paths of the files and directories in the specified directory.
/// If the path is not a directory, an `Error` is returned.
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// use crate::file_system::directory::get_data_from_dir;
///
/// let path = Path::new("/path/to/directory");
/// let result = get_data_from_dir(path);
/// match result {
///     Ok(data) => {
///         for path in data {
///             println!("{}", path.display());
///         }
///     }
///     Err(error) => {
///         eprintln!("Error: {}", error);
///     }
/// }
/// ```
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
