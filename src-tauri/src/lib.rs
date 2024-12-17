// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod datatypes;
mod tauface;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            tauface::save_meta,
            tauface::get_meta,
            tauface::get_home_data,
            tauface::get_day_history,
            tauface::get_analysis,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
