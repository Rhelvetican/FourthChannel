use config::Config;
use utils::HandlerResult;

mod config;
mod database;
mod utils;

#[tokio::main]
async fn main() -> HandlerResult<()> {
    let cfg = Config::default();

    Ok(())
}

#[cfg(test)]
mod test {
    use libjsonutils::file::write_json;

    use crate::config::Config;

    #[test]
    fn default_config() {
        write_json("./config/config.json", Config::default()).unwrap();
    }
}
