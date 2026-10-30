#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod storage;
mod imaging;
mod portable;

fn main() {
    tauri::Builder::default()
        .manage(storage::Locks::default())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![storage::open_session,storage::save_session,storage::release_sessions,storage::read_preferences,storage::write_preferences,storage::write_recovery,storage::read_recovery,storage::clear_recovery,imaging::inspect_image,imaging::rename_image,imaging::scan_folder,portable::export_package,portable::import_package])
        .run(tauri::generate_context!())
        .expect("Failed to run Copla");
}
