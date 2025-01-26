use tauri::Manager;

use crate::{
    datatypes::{Palette, PaletteData},
    tauface::Error,
};

#[tauri::command]
pub async fn save_palette(palette: PaletteData, app_handle: tauri::AppHandle) -> Result<(), Error> {
    let cache_dir = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|e| Error::Client(e.to_string()))?;
    if !cache_dir.exists() {
        std::fs::create_dir_all(&cache_dir).map_err(|e| Error::Client(e.to_string()))?;
    }
    let palette_path = cache_dir.join("palette.json");
    let palette_json = serde_json::to_string(&palette).map_err(|e| Error::Client(e.to_string()))?;
    std::fs::write(&palette_path, palette_json).map_err(|e| Error::Client(e.to_string()))?;
    Ok(())
}

#[tauri::command]
pub async fn get_palette(app_handle: tauri::AppHandle) -> Result<PaletteData, Error> {
    let cache_dir = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|e| Error::Client(e.to_string()))?;
    let palette_path = cache_dir.join("palette.json");
    if !palette_path.exists() {
        let palette = Palette {
            name: "Violet".to_string(),
            accent: "#3e0e3e".to_string(),
            accent_hover: "#efceff".to_string(),
            accent2: "#de9cff".to_string(),
            bg: "#200a2b".to_string(),
            bg_dark: "#1e1e1e".to_string(),
            disabled_color: "#fff7c3".to_string(),
        };
        let idx = 1;
        let palette_data = PaletteData { idx, palette };
        save_palette(palette_data, app_handle).await?;
    }
    let palette_json =
        std::fs::read_to_string(&palette_path).map_err(|e| Error::Client(e.to_string()))?;
    let palette_data: PaletteData =
        serde_json::from_str(&palette_json).map_err(|e| Error::Client(e.to_string()))?;
    Ok(palette_data)
}
