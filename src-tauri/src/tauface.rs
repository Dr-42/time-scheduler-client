use std::path::Path;

use chrono::{DateTime, Datelike, Local, NaiveDate, NaiveDateTime, NaiveTime};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha256::digest;
use tauri::Manager;

use crate::{
    datatypes::{
        AdjustTimeBlockQuery, AdjustTimeBlockQueryJs, Analysis, BlockType, CurrentBlock, HomeData,
        NewBlockType, Palette, PaletteData, SplitTimeBlockQuery, SplitTimeBlockQueryJs, SunHours,
        TimeBlock,
    },
    netutils::{make_get_request, make_post_request},
};

#[derive(Serialize, Deserialize)]
pub struct Meta {
    pub username: String,
    pub server_ip: String,
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Error {
    Client(String),
    Server(String),
    LoginExpired,
}

#[derive(Serialize, Debug)]
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

#[tauri::command]
pub async fn get_home_data(app_handle: tauri::AppHandle) -> Result<HomeData, Error> {
    let data_dir = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|e| Error::Client(e.to_string()))?;
    let mut home_data: HomeData = make_get_request("/state", &data_dir, None).await?;
    home_data.daydata.reverse();
    Ok(home_data)
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
    let data_dir = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|e| Error::Client(e.to_string()))?;
    let mut time_blocks: Vec<TimeBlock> = make_get_request(
        "/timeblock/get",
        &data_dir,
        Some(&[("date", &date.to_rfc3339())]),
    )
    .await?;
    time_blocks.reverse();
    let blocktypes = make_get_request("/blocktype/get", &data_dir, None).await?;
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
    let data_dir = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|e| Error::Client(e.to_string()))?;
    let analysis = make_get_request(
        "/analysis",
        &data_dir,
        Some(&[
            ("start", &start_date.to_rfc3339()),
            ("end", &end_date.to_rfc3339()),
        ]),
    )
    .await?;
    Ok(analysis)
}

#[tauri::command]
pub async fn post_next_block(
    data: CurrentBlock,
    app_handle: tauri::AppHandle,
) -> Result<(), Error> {
    let data_dir = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|e| Error::Client(e.to_string()))?;
    make_post_request("/timeblock/next", &data_dir, &data).await
}

#[tauri::command]
pub async fn post_split_block(
    app_handle: tauri::AppHandle,
    data: SplitTimeBlockQueryJs,
) -> Result<(), Error> {
    let data_dir = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|e| Error::Client(e.to_string()))?;
    let splits = data.split_time.split(":").collect::<Vec<&str>>();
    println!("{:?}", &data);
    let split_time = data
        .start_time
        .with_time(
            NaiveTime::from_hms_opt(
                splits[0]
                    .parse()
                    .map_err(|_| Error::Client("Failed to parse split time hour".to_string()))?,
                splits[1]
                    .parse()
                    .map_err(|_| Error::Client("Failed to parse split time minute".to_string()))?,
                splits[2]
                    .parse()
                    .map_err(|_| Error::Client("Failed to parse split time second".to_string()))?,
            )
            .ok_or(Error::Client("Failed to create split time".to_string()))?,
        )
        .single()
        .ok_or(Error::Client(
            "Failed to find unique split time".to_string(),
        ))?;
    let data = SplitTimeBlockQuery {
        start_time: data.start_time,
        end_time: data.end_time,
        split_time,
        before_title: data.before_title,
        after_title: data.after_title,
        before_block_type_id: data.before_block_type_id,
        after_block_type_id: data.after_block_type_id,
    };
    make_post_request("/timeblock/split", &data_dir, &data).await
}

#[tauri::command]
pub async fn post_adjust_block(
    app_handle: tauri::AppHandle,
    data: AdjustTimeBlockQueryJs,
) -> Result<(), Error> {
    let data_dir = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|e| Error::Client(e.to_string()))?;
    let new_start_splits = data.new_start_time.split(":").collect::<Vec<&str>>();
    let new_start_time = data
        .start_time
        .with_time(
            NaiveTime::from_hms_opt(
                new_start_splits[0].parse().map_err(|_| {
                    Error::Client("Failed to parse new start time hour".to_string())
                })?,
                new_start_splits[1].parse().map_err(|_| {
                    Error::Client("Failed to parse new start time minute".to_string())
                })?,
                new_start_splits[2].parse().map_err(|_| {
                    Error::Client("Failed to parse new start time second".to_string())
                })?,
            )
            .ok_or(Error::Client("Failed to create new start time".to_string()))?,
        )
        .single()
        .ok_or(Error::Client(
            "Failed to find unique new start time".to_string(),
        ))?;

    let new_end_splits = data.new_end_time.split(":").collect::<Vec<&str>>();
    let new_end_time = data
        .end_time
        .with_time(
            NaiveTime::from_hms_opt(
                new_end_splits[0]
                    .parse()
                    .map_err(|_| Error::Client("Failed to parse new end time hour".to_string()))?,
                new_end_splits[1].parse().map_err(|_| {
                    Error::Client("Failed to parse new end time minute".to_string())
                })?,
                new_end_splits[2].parse().map_err(|_| {
                    Error::Client("Failed to parse new end time second".to_string())
                })?,
            )
            .ok_or(Error::Client("Failed to create new end time".to_string()))?,
        )
        .single()
        .ok_or(Error::Client(
            "Failed to find unique new end time".to_string(),
        ))?;

    let data = AdjustTimeBlockQuery {
        start_time: data.start_time,
        end_time: data.end_time,
        new_start_time,
        new_end_time,
        title: data.title,
        block_type_id: data.block_type_id,
    };

    make_post_request("/timeblock/adjust", &data_dir, &data).await
}

#[tauri::command]
pub async fn post_change_current(
    data: CurrentBlock,
    app_handle: tauri::AppHandle,
) -> Result<(), Error> {
    let data_dir = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|e| Error::Client(e.to_string()))?;
    make_post_request("/currentblock/change", &data_dir, &data).await
}

#[tauri::command]
pub async fn post_new_block_type(
    data: NewBlockType,
    app_handle: tauri::AppHandle,
) -> Result<(), Error> {
    let data_dir = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|e| Error::Client(e.to_string()))?;
    make_post_request("/blocktype/new", &data_dir, &data).await
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
        .map_err(|e| Error::Client(e.to_string()))?;

    let local_data = local_data_response
        .text()
        .await
        .map_err(|e| Error::Client(e.to_string()))?;
    let local_body: Value =
        serde_json::from_str(&local_data).map_err(|e| Error::Client(e.to_string()))?;
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
        .ok_or(Error::Client("No public ip".to_string()))?;
    let locinfo = find(&public_ip.to_string()).await?;

    let lat = locinfo
        .latitude
        .parse::<f64>()
        .map_err(|_| Error::Client("Invalid latitude".to_string()))?;
    let long = locinfo
        .longitude
        .parse::<f64>()
        .map_err(|_| Error::Client("Invalid longitude".to_string()))?;

    let url = format!(
        "https://api.sunrisesunset.io/json?lat={}&lng={}&formatted=0&timezone=Asia/Kolkata",
        lat, long
    );

    // Make the HTTP request
    let response = reqwest::get(&url)
        .await
        .map_err(|e| Error::Client(e.to_string()))?
        .json::<SunApiResponse>()
        .await
        .map_err(|e| Error::Client(e.to_string()))?;

    // Check API response status
    if response.status != "OK" {
        return Err(Error::Client("Failed to retrieve sun hours".to_string()));
    }

    let today = Local::now().date_naive();
    let date = NaiveDate::from_ymd_opt(today.year(), today.month(), today.day())
        .ok_or(Error::Client("Invalid date".to_string()))?;

    let final_sunrise_str = format!("{} {}", date, response.results.sunrise);
    let final_sunset_str = format!("{} {}", date, response.results.sunset);

    let sunrise_local = NaiveDateTime::parse_from_str(&final_sunrise_str, "%Y-%m-%d %I:%M:%S %p")
        .map_err(|e| Error::Client(e.to_string()))?
        .and_local_timezone(Local)
        .single()
        .ok_or(Error::Client("Failed to parse sunrise time".to_string()))?;
    let sunset_local = NaiveDateTime::parse_from_str(&final_sunset_str, "%Y-%m-%d %I:%M:%S %p")
        .map_err(|e| Error::Client(e.to_string()))?
        .and_local_timezone(Local)
        .single()
        .ok_or(Error::Client("Failed to parse sunset time".to_string()))?;

    Ok(SunHours {
        sunrise: sunrise_local,
        sunset: sunset_local,
    })
}

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
