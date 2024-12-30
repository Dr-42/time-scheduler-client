// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#![deny(clippy::unwrap_used, clippy::expect_used)]

mod datatypes;
mod tauface;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            tauface::save_meta,
            tauface::get_meta,
            tauface::get_home_data,
            tauface::get_day_history,
            tauface::get_analysis,
            tauface::post_next_block,
            tauface::post_change_current,
            tauface::post_new_block_type,
        ])
        .run(tauri::generate_context!())?;
    Ok(())
}
