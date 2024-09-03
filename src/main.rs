use config::Config;
use database::Database;
use libjsonutils::file::write_json;
use teloxide::{prelude::Dispatcher, Bot};
use utils::Result;

mod config;
mod core;
mod database;
mod handler;
mod utils;

const CONFIG_PATH: &str = "./config/config.json";

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = match Config::load() {
        Ok(cfg) => cfg,
        Err(_) => {
            let def_cfg = Config::default();
            let _ = write_json(CONFIG_PATH, &def_cfg);

            println!("No config file found. Generated default config file.");
            return Ok(());
        }
    };

    let db = Database::open(&cfg.database.database_file)?;
    let bot = Bot::new(&cfg.telegram.token);

    Dispatcher::builder(bot, handler::handlers())
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}
