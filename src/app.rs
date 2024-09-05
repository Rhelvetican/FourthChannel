use crate::{
    config::Config,
    db::{Database, DatabaseProvider},
    utils::Result,
};

pub async fn app(cfg: &Config) -> Result<()> {
    let db = cfg.database.database_file.clone();

    let db = Database::with_provider(if db.exists() {
        DatabaseProvider::InFile(db)
    } else {
        DatabaseProvider::InMem
    })?;

    Ok(())
}
