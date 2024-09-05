use config::Config;
use libjsonutils::file::write_json;
use teloxide::{dptree::deps, prelude::Dispatcher, Bot};
use utils::Result;

mod app;
mod config;
mod db;
mod handler;
mod modules;
mod utils;

const CONFIG_PATH: &str = "./config/config.json";

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = match Config::load() {
        Ok(cfg) => cfg,
        Err(_) => Config::init(),
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
