use config::Config;
use libjsonutils::file::write_json;
use teloxide::{dptree::deps, prelude::Dispatcher, Bot};
use utils::Result;

mod config;
mod core;
mod db;
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

    let bot = Bot::new(&cfg.telegram.token);

    Dispatcher::builder(bot, handler::handlers())
        .dependencies(deps![])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}
