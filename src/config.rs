use std::fs::{create_dir, exists, File};

use serde::{Deserialize, Serialize};
use serde_json::{from_reader, ser::PrettyFormatter, Serializer};

use crate::util::Res;

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub name: String,
    pub database: DatabaseConfig,
    pub media: MediaConfig,
    pub policies: Policies,
    pub telegram: TelegramConfig,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DatabaseConfig {
    pub database_file: String,
    pub media_folder: String,
    pub owner_id: u64,
    pub post_id: u64,
    pub post_username: String,
    pub seed: i64,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MediaConfig {
    pub auto_purge_interval: u64,
    pub auto_purge_media: bool,
    pub max_image_size: u64,
    pub max_video_size: u64,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Policies {
    pub auto_delete_like_limit: i64,
    pub auto_delete_queue_length: i64,
    pub delete_dislike_limit: i64,
    pub pin_like_limit: i64,
    pub post_interval: i64,
    pub unpin_dislike_limit: i64,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TelegramConfig {
    pub api_hash: String,
    pub api_id: i64,
    pub token: String,
    pub username: String,
}

impl Config {
    pub fn load() -> Res<Config> {
        Ok(from_reader(File::open("./config/config.json")?)?)
    }

    pub fn init() -> Res<()> {
        if !exists("./config/")? {
            create_dir("./config/")?
        }

        let mut cfg_file = File::create("./config/config.json")?;
        let def = Config::default();

        let fmt = PrettyFormatter::with_indent(b"    ");
        let mut ser = Serializer::with_formatter(&mut cfg_file, fmt);

        def.serialize(&mut ser)?;

        Ok(())
    }
}
