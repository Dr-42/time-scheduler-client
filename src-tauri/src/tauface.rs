use chrono::{DateTime, Local};
use reqwest::{header::AUTHORIZATION, Client};
use serde::{Deserialize, Serialize};

use crate::datatypes::{Analysis, HomeData, TimeBlock};

#[derive(Serialize, Deserialize)]
pub struct Meta {
    pub username: String,
    pub pass_hash: String,
    pub server_ip: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Error {
    ClientError(String),
    ServerError(String),
}

#[tauri::command]
pub async fn save_meta(
    username: Option<&str>,
    pass_hash: Option<&str>,
    server_ip: Option<&str>,
) -> Result<(), Error> {
    let dirs = directories::ProjectDirs::from("org", "dr42", "timescheduler")
        .ok_or(Error::ClientError("Directories doesn't work".to_string()))?;
    let cache_dir = dirs.cache_dir().to_owned();
    if !cache_dir.exists() {
        std::fs::create_dir_all(&cache_dir).map_err(|e| Error::ClientError(e.to_string()))?;
    }
    let meta_path = cache_dir.join("meta.json");

    let loaded_meta = match std::fs::read_to_string(&meta_path) {
        Ok(meta) => {
            let meta: Meta = serde_json::from_str(&meta)
                .map_err(|e| Error::ClientError(format!("Invalid meta file: {}", e)))?;
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

    let meta_json = serde_json::to_string(&meta).map_err(|e| Error::ClientError(e.to_string()))?;
    std::fs::write(meta_path, meta_json).map_err(|e| Error::ClientError(e.to_string()))?;
    Ok(())
}

#[tauri::command]
pub async fn get_meta() -> Result<Meta, Error> {
    let dirs = directories::ProjectDirs::from("org", "dr42", "timescheduler")
        .ok_or(Error::ClientError("Directories doesn't work".to_string()))?;
    let cache_dir = dirs.cache_dir().to_owned();
    let meta_path = cache_dir.join("meta.json");
    let meta_json =
        std::fs::read_to_string(&meta_path).map_err(|e| Error::ClientError(e.to_string()))?;
    let meta: Meta =
        serde_json::from_str(&meta_json).map_err(|e| Error::ClientError(e.to_string()))?;
    Ok(meta)
}

#[tauri::command]
pub async fn get_home_data() -> Result<HomeData, Error> {
    let meta = get_meta().await?;

    let client = Client::new();
    // Recieve a json from the server
    let response = client
        .get(format!("http://{}/state", meta.server_ip))
        .header(AUTHORIZATION, format!("Bearer {}", meta.pass_hash))
        .send()
        .await
        .map_err(|e| Error::ServerError(e.to_string()))?;

    if !response.status().is_success() {
        let e = response
            .text()
            .await
            .map_err(|e| Error::ClientError(e.to_string()))?;
        return Err(Error::ServerError(e));
    }

    let mut response = response
        .json::<HomeData>()
        .await
        .map_err(|e| Error::ClientError(e.to_string()))?;

    response.daydata.reverse();

    Ok(response)
}

#[tauri::command]
pub async fn get_day_history(date: DateTime<Local>) -> Result<Vec<TimeBlock>, Error> {
    let meta = get_meta().await?;
    let client = Client::new();
    let response = client
        .get(format!("http://{}/daydata", meta.server_ip))
        .query(&[("date", date.to_rfc3339())])
        .header(AUTHORIZATION, format!("Bearer {}", meta.pass_hash))
        .send()
        .await
        .map_err(|e| Error::ServerError(e.to_string()))?;

    println!("{:?}", response);

    if !response.status().is_success() {
        let e = response
            .text()
            .await
            .map_err(|e| Error::ClientError(e.to_string()))?;
        return Err(Error::ServerError(e));
    }

    let response = response
        .json::<Vec<TimeBlock>>()
        .await
        .map_err(|e| Error::ClientError(e.to_string()))?;

    Ok(response)
}

#[tauri::command]
pub async fn get_analysis(
    start_date: DateTime<Local>,
    end_date: DateTime<Local>,
) -> Result<Analysis, Error> {
    let meta = get_meta().await?;
    let client = Client::new();
    let response = client
        .get(format!("http://{}/analysis", meta.server_ip))
        .query(&[
            ("start", start_date.to_rfc3339()),
            ("end", end_date.to_rfc3339()),
        ])
        .header(AUTHORIZATION, format!("Bearer {}", meta.pass_hash))
        .send()
        .await
        .map_err(|e| Error::ServerError(e.to_string()))?
        .json::<Analysis>()
        .await
        .map_err(|e| Error::ClientError(e.to_string()))?;

    Ok(response)
}
