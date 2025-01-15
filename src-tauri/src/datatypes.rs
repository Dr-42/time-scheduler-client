use std::time::Duration;

use chrono::{DateTime, Local, NaiveDate};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct BlockType {
    pub id: u8,
    pub name: String,
    pub color: Color,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TimeBlock {
    pub start_time: DateTime<Local>,
    pub end_time: DateTime<Local>,
    pub block_type_id: u8,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentBlock {
    pub block_type_id: u8,
    pub current_block_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Trend {
    pub day: NaiveDate,
    pub time_spent: Duration,
    pub block_type_id: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Analysis {
    pub percentages: Vec<f32>,
    pub trends: Vec<Trend>,
    pub blocktypes: Vec<BlockType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HomeData {
    pub blocktypes: Vec<BlockType>,
    pub daydata: Vec<TimeBlock>,
    pub currentblock: CurrentBlock,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewBlockType {
    name: String,
    color: Color,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SunHours {
    pub sunrise: DateTime<Local>,
    pub sunset: DateTime<Local>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Palette {
    pub name: String,
    pub accent: String,
    pub accent_hover: String,
    pub accent2: String,
    pub bg: String,
    pub bg_dark: String,
    pub disabled_color: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaletteData {
    pub idx: u8,
    pub palette: Palette,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SplitTimeBlockQuery {
    pub start_time: DateTime<Local>,
    pub end_time: DateTime<Local>,
    pub split_time: DateTime<Local>,
    pub before_title: String,
    pub after_title: String,
    pub before_block_type_id: u8,
    pub after_block_type_id: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SplitTimeBlockQueryJs {
    pub start_time: DateTime<Local>,
    pub end_time: DateTime<Local>,
    pub split_time: String,
    pub before_title: String,
    pub after_title: String,
    pub before_block_type_id: u8,
    pub after_block_type_id: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AdjustTimeBlockQuery {
    pub start_time: DateTime<Local>,
    pub end_time: DateTime<Local>,
    pub new_start_time: DateTime<Local>,
    pub new_end_time: DateTime<Local>,
    pub title: String,
    pub block_type_id: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AdjustTimeBlockQueryJs {
    pub start_time: DateTime<Local>,
    pub end_time: DateTime<Local>,
    pub new_start_time: String,
    pub new_end_time: String,
    pub title: String,
    pub block_type_id: u8,
}
