use chrono::{DateTime, Datelike, Local, NaiveDate, NaiveDateTime};
use reqwest::{header::AUTHORIZATION, Client};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha256::digest;
use tauri::Manager;

use crate::datatypes::{
    Analysis, BlockType, CurrentBlock, HomeData, NewBlockType, SunHours, TimeBlock,
};

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
    password: Option<&str>,
    server_ip: Option<&str>,
    app_handle: tauri::AppHandle,
) -> Result<(), Error> {
    let cache_dir = app_handle
        .path()
        .app_cache_dir()
        .map_err(|e| Error::ClientError(e.to_string()))?;
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

    let hashed_pass = if let Some(password) = password {
        digest(password)
    } else {
        loaded_meta.pass_hash.clone()
    };

    let meta = Meta {
        username: username.unwrap_or(&loaded_meta.username).to_string(),
        pass_hash: hashed_pass,
        server_ip: server_ip.unwrap_or(&loaded_meta.server_ip).to_string(),
    };

    let meta_json = serde_json::to_string(&meta).map_err(|e| Error::ClientError(e.to_string()))?;
    std::fs::write(meta_path, meta_json).map_err(|e| Error::ClientError(e.to_string()))?;
    Ok(())
}

#[tauri::command]
pub async fn get_meta(app_handle: tauri::AppHandle) -> Result<Meta, Error> {
    let cache_dir = app_handle
        .path()
        .app_cache_dir()
        .map_err(|e| Error::ClientError(e.to_string()))?;
    let meta_path = cache_dir.join("meta.json");
    let meta_json =
        std::fs::read_to_string(&meta_path).map_err(|e| Error::ClientError(e.to_string()))?;
    let meta: Meta =
        serde_json::from_str(&meta_json).map_err(|e| Error::ClientError(e.to_string()))?;
    Ok(meta)
}

#[tauri::command]
pub async fn get_home_data(app_handle: tauri::AppHandle) -> Result<HomeData, Error> {
    let meta = get_meta(app_handle).await?;

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

#[derive(Serialize, Deserialize)]
pub struct HistoryData {
    pub daydata: Vec<TimeBlock>,
    pub blocktypes: Vec<BlockType>,
}

#[tauri::command]
pub async fn get_day_history(
    date: DateTime<Local>,
    app_handle: tauri::AppHandle,
) -> Result<HistoryData, Error> {
    let meta = get_meta(app_handle).await?;
    let client = Client::new();
    let time_blocks_response = client
        .get(format!("http://{}/daydata", meta.server_ip))
        .query(&[("date", date.to_rfc3339())])
        .header(AUTHORIZATION, format!("Bearer {}", meta.pass_hash))
        .send()
        .await
        .map_err(|e| Error::ServerError(e.to_string()))?;

    if !time_blocks_response.status().is_success() {
        let e = time_blocks_response
            .text()
            .await
            .map_err(|e| Error::ClientError(e.to_string()))?;
        return Err(Error::ServerError(e));
    }

    let mut time_blocks = time_blocks_response
        .json::<Vec<TimeBlock>>()
        .await
        .map_err(|e| Error::ClientError(e.to_string()))?;

    time_blocks.reverse();

    let blocktypes_response = client
        .get(format!("http://{}/blocktypes", meta.server_ip))
        .header(AUTHORIZATION, format!("Bearer {}", meta.pass_hash))
        .send()
        .await
        .map_err(|e| Error::ServerError(e.to_string()))?;

    if !blocktypes_response.status().is_success() {
        let e = blocktypes_response
            .text()
            .await
            .map_err(|e| Error::ClientError(e.to_string()))?;
        return Err(Error::ServerError(e));
    }

    let blocktypes = blocktypes_response
        .json::<Vec<BlockType>>()
        .await
        .map_err(|e| Error::ClientError(e.to_string()))?;

    let res = HistoryData {
        daydata: time_blocks,
        blocktypes,
    };

    Ok(res)
}

#[tauri::command]
pub async fn get_analysis(
    start_date: DateTime<Local>,
    end_date: DateTime<Local>,
    app_handle: tauri::AppHandle,
) -> Result<Analysis, Error> {
    let meta = get_meta(app_handle).await?;
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

#[tauri::command]
pub async fn post_next_block(
    data: CurrentBlock,
    app_handle: tauri::AppHandle,
) -> Result<(), Error> {
    let meta = get_meta(app_handle).await?;
    let client = Client::new();
    let response = client
        .post(format!("http://{}/nexttimeblock", meta.server_ip))
        .header(AUTHORIZATION, format!("Bearer {}", meta.pass_hash))
        .json(&data)
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

    Ok(())
}

#[tauri::command]
pub async fn post_change_current(
    data: CurrentBlock,
    app_handle: tauri::AppHandle,
) -> Result<(), Error> {
    let meta = get_meta(app_handle).await?;
    let client = Client::new();
    let response = client
        .post(format!("http://{}/changecurrentblock", meta.server_ip))
        .header(AUTHORIZATION, format!("Bearer {}", meta.pass_hash))
        .json(&data)
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

    Ok(())
}

#[tauri::command]
pub async fn post_new_block_type(
    data: NewBlockType,
    app_handle: tauri::AppHandle,
) -> Result<(), Error> {
    let meta = get_meta(app_handle).await?;
    let client = Client::new();
    let response = client
        .post(format!("http://{}/newblocktype", meta.server_ip))
        .header(AUTHORIZATION, format!("Bearer {}", meta.pass_hash))
        .json(&data)
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

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Locator {
    pub ip: String,
    pub latitude: String,
    pub longitude: String,
    pub city: String,
    pub region: String,
    pub country: String,
    pub timezone: String,
    pub location: String,
}

pub async fn find(ip: &str) -> Result<Locator, Error> {
    let uri = format!("http://ip-api.com/json/{}", &ip);

    let client = Client::new();
    let local_data_response = client
        .get(&uri)
        .send()
        .await
        .map_err(|e| Error::ClientError(e.to_string()))?;

    let local_data = local_data_response
        .text()
        .await
        .map_err(|e| Error::ClientError(e.to_string()))?;
    let local_body: Value =
        serde_json::from_str(&local_data).map_err(|e| Error::ClientError(e.to_string()))?;
    let result = Locator {
        ip: local_body["query"].to_string(),
        latitude: local_body["lat"].to_string(),
        longitude: local_body["lon"].to_string(),
        city: local_body["city"].to_string(),
        region: local_body["regionName"].to_string(),
        country: local_body["country"].to_string(),
        timezone: local_body["timezone"].to_string(),
        location: format!(
            "{:?}, {:?}, {:?}",
            local_body["city"].to_string(),
            local_body["regionName"].to_string(),
            local_body["country"].to_string()
        ),
    };

    Ok(result)
}

#[derive(Deserialize, Debug)]
struct SunApiResponse {
    results: SunApiResults,
    status: String,
}

#[derive(Deserialize, Debug)]
struct SunApiResults {
    sunrise: String,
    sunset: String,
}

#[tauri::command]
pub async fn get_sun_hours() -> Result<SunHours, Error> {
    let public_ip = public_ip::addr()
        .await
        .ok_or(Error::ClientError("No public ip".to_string()))?;
    let locinfo = find(&public_ip.to_string()).await?;

    let lat = locinfo
        .latitude
        .parse::<f64>()
        .map_err(|_| Error::ClientError("Invalid latitude".to_string()))?;
    let long = locinfo
        .longitude
        .parse::<f64>()
        .map_err(|_| Error::ClientError("Invalid longitude".to_string()))?;

    let url = format!(
        "https://api.sunrisesunset.io/json?lat={}&lng={}&formatted=0&timezone=Asia/Kolkata",
        lat, long
    );

    // Make the HTTP request
    let response = reqwest::get(&url)
        .await
        .map_err(|e| Error::ClientError(e.to_string()))?
        .json::<SunApiResponse>()
        .await
        .map_err(|e| Error::ClientError(e.to_string()))?;

    // Check API response status
    if response.status != "OK" {
        return Err(Error::ClientError(
            "Failed to retrieve sun hours".to_string(),
        ));
    }

    let today = Local::now().date_naive();
    let date = NaiveDate::from_ymd_opt(today.year(), today.month(), today.day())
        .ok_or(Error::ClientError("Invalid date".to_string()))?;

    let final_sunrise_str = format!("{} {}", date, response.results.sunrise);
    let final_sunset_str = format!("{} {}", date, response.results.sunset);

    let sunrise_local = NaiveDateTime::parse_from_str(&final_sunrise_str, "%Y-%m-%d %I:%M:%S %p")
        .map_err(|e| Error::ClientError(e.to_string()))?
        .and_local_timezone(Local)
        .single()
        .ok_or(Error::ClientError(
            "Failed to parse sunrise time".to_string(),
        ))?;
    let sunset_local = NaiveDateTime::parse_from_str(&final_sunset_str, "%Y-%m-%d %I:%M:%S %p")
        .map_err(|e| Error::ClientError(e.to_string()))?
        .and_local_timezone(Local)
        .single()
        .ok_or(Error::ClientError(
            "Failed to parse sunset time".to_string(),
        ))?;

    Ok(SunHours {
        sunrise: sunrise_local,
        sunset: sunset_local,
    })
}
