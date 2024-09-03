use std::{error::Error as StdErr, fmt::Display, path::Path, result::Result as StdRes};

use tokio::{
    fs::{create_dir, File},
    io::AsyncWriteExt,
};

pub type Error = Box<dyn StdErr + Send + Sync>;
pub type Result<T> = StdRes<T, Error>;

pub struct Log {
    log: File,
}

impl Log {
    pub async fn new() -> Self {
        if !Path::new("./logs/").exists() {
            create_dir("./logs").await.unwrap()
        };

        let log = if !Path::new("./logs/fc.log").exists() {
            File::create_new("./logs/fc.log").await.unwrap()
        } else {
            File::create("./logs/fc.log").await.unwrap()
        };

        Self { log }
    }

    pub async fn write<M: Display>(&mut self, msg: M) -> Result<()> {
        self.log
            .write_all(msg.to_string().as_bytes())
            .await
            .map_err(|e| e.into())
    }
}
