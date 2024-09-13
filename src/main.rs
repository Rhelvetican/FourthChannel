use std::sync::Arc;

use config::Config;
use db::{Database, InFileProvider, InMemProvider};
use teloxide::{
    prelude::{DependencyMap, Dispatcher},
    Bot,
};
use util::Res;

mod config;
mod db;
mod util;

#[tokio::main]
async fn main() -> Res<()> {
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

    let mut deps = DependencyMap::new();
    deps.insert(cfg);

    Ok(())
}
