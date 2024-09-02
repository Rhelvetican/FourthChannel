use config::Config;
use libjsonutils::file::write_json;
use utils::Result;

mod config;
mod core;
mod database;
mod utils;

const CONFIG_PATH: &str = "./config/config.json";

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = match Config::load() {
        Ok(cfg) => cfg,
        Err(_) => {
            let def_cfg = Config::default();
            let _ = write_json(CONFIG_PATH, &def_cfg);
            return Ok(());
        }
    };

    Ok(())
}
