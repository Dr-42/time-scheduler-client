use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::datatypes::HomeData;

#[derive(Serialize, Deserialize)]
pub struct Meta {
    username: String,
    pass_hash: String,
    server_ip: String,
}

#[tauri::command]
pub async fn save_meta(
    username: Option<&str>,
    pass_hash: Option<&str>,
    server_ip: Option<&str>,
) -> Result<(), String> {
    let dirs = directories::ProjectDirs::from("org", "dr42", "timescheduler")
        .ok_or("Directories doesn't work")?;
    let cache_dir = dirs.cache_dir().to_owned();
    let meta_path = cache_dir.join("meta.json");

    let loaded_meta = match std::fs::read_to_string(&meta_path) {
        Ok(meta) => {
            let meta: Meta = serde_json::from_str(&meta).unwrap();
            meta
        }
        Err(_) => Meta {
            username: "".to_string(),
            pass_hash: "".to_string(),
            server_ip: "".to_string(),
        },
    };

    let meta = Meta {
        username: username.unwrap_or(&loaded_meta.username).to_string(),
        pass_hash: pass_hash.unwrap_or(&loaded_meta.pass_hash).to_string(),
        server_ip: server_ip.unwrap_or(&loaded_meta.server_ip).to_string(),
    };

    let meta_json = serde_json::to_string(&meta).unwrap();
    std::fs::write(meta_path, meta_json).unwrap();
    Ok(())
}

#[tauri::command]
pub async fn get_meta() -> Meta {
    let dirs = directories::ProjectDirs::from("org", "dr42", "timescheduler")
        .ok_or("Directories doesn't work")
        .unwrap();
    let cache_dir = dirs.cache_dir().to_owned();
    let meta_path = cache_dir.join("meta.json");
    let meta_json = std::fs::read_to_string(&meta_path).unwrap();
    let meta: Meta = serde_json::from_str(&meta_json).unwrap();
    meta
}

#[tauri::command]
pub async fn get_home_data() -> Result<HomeData, String> {
    todo!()
}

#[tauri::command]
pub async fn get_day_history(date: NaiveDate) -> Result<HomeData, String> {
    todo!()
}

#[tauri::command]
pub async fn get_analysis(start_date: NaiveDate, end_date: NaiveDate) -> Result<HomeData, String> {
    todo!()
}
