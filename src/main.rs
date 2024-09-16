use std::sync::{Arc, Mutex};

use config::Config;
use db::{Database, InFileProvider, InMemProvider};

use handler::handler;
use teloxide::{
    prelude::{DependencyMap, Dispatcher},
    Bot,
};
use util::Result;

mod config;
mod db;
mod handler;
mod util;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let cfg = match Config::load() {
        Ok(cfg) => cfg,
        Err(e) => {
            Config::init()?;

            println!("Unable to load config. Generated a default one.");
            return Err(e);
        }
    };

    let cfg = Arc::new(cfg);

    let db = if !cfg.database.database_file.is_empty() {
        Database::from_provider(InFileProvider::new(&*cfg.database.database_file))
    } else {
        Database::from_provider(InMemProvider)
    }?;

    db.migrations()?;

    let bot = Bot::new(&cfg.telegram.token);
    let db = Arc::new(Mutex::new(db));

    let mut deps = DependencyMap::new();
    deps.insert(cfg);
    deps.insert(db);

    Dispatcher::builder(bot, handler())
        .dependencies(deps)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}
