use fourth_channel::{config::Config, util::Result};

#[tokio::main]
async fn main() -> Result<()> {
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
