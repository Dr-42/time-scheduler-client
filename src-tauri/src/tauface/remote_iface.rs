use chrono::{DateTime, Local, NaiveTime};
use serde::{Deserialize, Serialize};
use tauri::Manager;

use crate::{
    datatypes::{
        AdjustTimeBlockQuery, AdjustTimeBlockQueryJs, Analysis, BlockType, CurrentBlock, HomeData,
        NewBlockType, SplitTimeBlockQuery, SplitTimeBlockQueryJs, TimeBlock,
    },
    netutils::{make_get_request, make_post_request},
};

use crate::error::Error;

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
