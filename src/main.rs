use config::Config;
use util::Res;

mod config;
mod util;

#[tokio::main]
async fn main() -> Res<()> {
    let cfg = match Config::load() {
        Ok(cfg) => cfg,
        Err(e) => {
            Config::init()?;

            println!("Unable to load config. Generated a default one.");
            return Err(e);
        }
    };

    Ok(())
}
