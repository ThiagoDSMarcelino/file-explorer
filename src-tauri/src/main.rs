#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::file_system::storage::get_all_storages;

mod file_system;
mod prelude;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_all_storages])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}
