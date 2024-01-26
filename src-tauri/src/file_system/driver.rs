use serde::Serialize;
use sysinfo::{Disk, Disks};

use crate::file_system::bytes_converter::bytes_to_gigabytes;

/// Represents a driver in the file system.
#[derive(Debug, Serialize)]
pub(crate) struct Driver {
    name: String,
    kind: String,
    dir: String,
    file_system: String,
    total_space: f64,
    used_space: f64,
}

impl Driver {
    /// Creates a new `Driver` instance from a `Disk` object.
    ///
    /// # Arguments
    ///
    /// * `disk` - The `Disk` object to create the `Driver` from.
    ///
    /// # Returns
    ///
    /// A new `Driver` instance.
    fn from_disk(disk: &Disk) -> Self {
        let space = disk.total_space();

        Self {
            name: disk.name().to_str().unwrap().to_string(),
            kind: disk.kind().to_string(),
            dir: disk.mount_point().to_str().unwrap().to_string(),
            file_system: disk.file_system().to_str().unwrap().to_string(),
            total_space: bytes_to_gigabytes(space),
            used_space: bytes_to_gigabytes(space - disk.available_space()),
        }
    }
}

/// Retrieves information about all drivers in the system.
///
/// # Returns
///
/// A vector of `Driver` instances representing all the drivers in the system.
#[tauri::command]
pub fn get_all_drivers() -> Vec<Driver> {
    let disks = Disks::new_with_refreshed_list();

    disks.list().iter().map(Driver::from_disk).collect()
}
