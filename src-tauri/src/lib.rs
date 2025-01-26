// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#![deny(clippy::unwrap_used, clippy::expect_used)]

mod datatypes;
mod error;
mod netutils;
mod tauface;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            tauface::meta::save_meta,
            tauface::meta::get_meta,
            tauface::remote_iface::get_home_data,
            tauface::remote_iface::get_day_history,
            tauface::remote_iface::get_analysis,
            tauface::remote_iface::post_next_block,
            tauface::remote_iface::post_split_block,
            tauface::remote_iface::post_adjust_block,
            tauface::remote_iface::post_change_current,
            tauface::remote_iface::post_new_block_type,
            tauface::sun::get_sun_hours,
            tauface::pallete::get_palette,
            tauface::pallete::save_palette
        ])
        .run(tauri::generate_context!())?;
    Ok(())
}
