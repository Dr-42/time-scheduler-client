use chrono::{Datelike, Local, NaiveDate, NaiveDateTime};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::datatypes::SunHours;

use super::Error;

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
