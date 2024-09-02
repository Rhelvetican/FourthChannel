use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Config {
    pub name: String,
    pub telegram: TelegramConfig,
    pub database: DatabaseConfig,
    pub policies: Policies,
    pub media: MediaConfig,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TelegramConfig {
    pub api_id: u64,
    pub api_hash: String,
    pub token: String,
    pub username: String,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseConfig {
    pub seed: u64,
    pub owner_id: u64,
    pub database_file: PathBuf,
    pub post_id: i64,
    pub media_folder: PathBuf,
    pub post_username: String,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Policies {
    pub post_interval: u64,
    pub delete_dislike_limit: i32,
    pub unpin_dislike_limit: i32,
    pub auto_delete_like_limit: i32,
    pub pin_like_limit: i32,
    pub auto_delete_queue_length: usize,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaConfig {
    pub max_video_size: u32,
    pub max_image_size: u32,
    pub auto_purge_media: bool,
    pub auto_purge_interval: u64,
}
