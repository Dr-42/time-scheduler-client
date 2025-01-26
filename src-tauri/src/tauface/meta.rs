use std::path::Path;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use sha256::digest;
use tauri::Manager;

use crate::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    pub username: String,
    pub server_ip: String,
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequest {
    pub key: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[tauri::command]
pub async fn save_meta(
    username: &str,
    password: &str,
    server_ip: &str,
    app_handle: tauri::AppHandle,
) -> Result<(), Error> {
    let data_dir = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|e| Error::Client(e.to_string()))?;
    if !data_dir.exists() {
        std::fs::create_dir_all(&data_dir).map_err(|e| Error::Client(e.to_string()))?;
    }
    let meta_path = data_dir.join("meta.json");

    let hashed_pass = digest(password);

    let login_req = LoginRequest { key: hashed_pass };
    let client = Client::new();
    let response = client
        .post(format!("http://{}/auth/login", server_ip))
        .json(&login_req)
        .send()
        .await
        .map_err(|e| Error::Server(e.to_string()))?
        .json::<LoginResponse>()
        .await
        .map_err(|e| Error::Client(e.to_string()))?;
    println!("{:?}", response);

    let meta = Meta {
        username: username.to_string(),
        server_ip: server_ip.to_string(),
        access_token: response.access_token,
        refresh_token: response.refresh_token,
    };

    let meta_json = serde_json::to_string(&meta).map_err(|e| Error::Client(e.to_string()))?;
    std::fs::write(meta_path, meta_json).map_err(|e| Error::Client(e.to_string()))?;
    Ok(())
}

#[tauri::command]
pub async fn get_meta(app_handle: tauri::AppHandle) -> Result<Meta, Error> {
    let data_dir = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|e| Error::Client(e.to_string()))?;
    get_meta_internal(&data_dir).await
}

pub async fn get_meta_internal(data_dir: &Path) -> Result<Meta, Error> {
    let meta_path = data_dir.join("meta.json");
    let meta_json =
        std::fs::read_to_string(&meta_path).map_err(|e| Error::Client(e.to_string()))?;
    let meta: Meta = serde_json::from_str(&meta_json).map_err(|e| Error::Client(e.to_string()))?;
    Ok(meta)
}
