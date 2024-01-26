#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::file_system::directory::get_data_from_dir;
use crate::file_system::driver::get_all_drivers;

mod error;
mod file_system;
mod prelude;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_all_drivers, get_data_from_dir])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}
