use std::{fs::File, io::BufReader, path::PathBuf};

use libjsonutils::file::write_json;
use serde::{Deserialize, Serialize};
use serde_json::from_reader;

use crate::{utils::Result, CONFIG_PATH};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Config {
    pub name: String,
    pub telegram: TelegramConfig,
    pub database: DatabaseConfig,
    pub policies: Policies,
    pub media: MediaConfig,
}

impl Config {
    pub fn load() -> Result<Self> {
        let f = File::open(CONFIG_PATH)?;
        let rdr = BufReader::new(f);
        from_reader(rdr).map_err(|e| e.into())
    }

    pub fn init() -> Self {
        let def_cfg = Config::default();
        let _ = write_json(CONFIG_PATH, &def_cfg);

        println!("No config file found. Generated default config file.");
        def_cfg
    }
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
