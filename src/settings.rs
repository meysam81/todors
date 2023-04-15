use config::{Config, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    #[serde(default = "default_dburl")]
    pub db_url: String,
}

impl Settings {
    pub fn new() -> Result<Self, config::ConfigError> {
        let s = Config::builder()
            .add_source(Environment::default())
            .build()?;
        s.try_deserialize()
    }
}

fn default_dburl() -> String {
    let user_home = dirs::home_dir().unwrap();
    let path = user_home.join(".todors");

    if let Err(e) = std::fs::create_dir_all(&path) {
        eprintln!("Failed to create directory: {}", e);
    }

    let path = path.join("db.sqlite");
    path.to_str().unwrap().to_string()
}
