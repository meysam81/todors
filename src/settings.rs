use crate::serializers::Deserialize;
use config::{Config, Environment};

#[derive(Debug, Deserialize)]
pub struct Settings {
    #[serde(default = "default_dburl")]
    pub db_url: String,
    #[serde(default = "default_loglevel")]
    pub log_level: String,
    #[serde(default = "num_cpus::get")]
    pub num_workers: usize,
    #[serde(default = "default_pagination_limit")]
    pub pagination_limit: u32,
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

fn default_loglevel() -> String {
    "info".to_string()
}

fn default_pagination_limit() -> u32 {
    100
}
