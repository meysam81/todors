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
    #[serde(default = "default_pagination_hard_limit")]
    pub pagination_hard_limit: u32,
    #[serde(default = "default_create_batch_hard_limit")]
    pub create_batch_hard_limit: u32,
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

    let path = path.join("db.sqlite");
    path.to_str().unwrap().to_string()
}

fn default_loglevel() -> String {
    crate::consts::DEFAULT_LOG_LEVEL.to_string()
}

fn default_pagination_limit() -> u32 {
    crate::consts::DEFAULT_PAGE_LIMIT
}

fn default_pagination_hard_limit() -> u32 {
    crate::consts::DEFAULT_PAGE_HARD_LIMIT
}

fn default_create_batch_hard_limit() -> u32 {
    crate::consts::DEFAULT_CREATE_BATCH_HARD_LIMIT
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env;

    #[test]
    #[serial]
    fn db_url_will_take_default_if_env_is_not_provided() {
        let old_db_url = env::var("DB_URL");
        env::remove_var("DB_URL");
        let settings = Settings::new().unwrap();
        assert_eq!(settings.db_url, default_dburl());
        if let Ok(db_url) = old_db_url {
            env::set_var("DB_URL", db_url);
        }
    }

    #[test]
    #[serial]
    fn db_url_is_overriden_from_env() {
        let old_db_url = env::var("DB_URL");
        let db_url = "sqlite://:memory:";
        env::set_var("DB_URL", db_url);
        let settings = Settings::new().unwrap();
        assert_eq!(settings.db_url, db_url);
        if let Ok(db_url) = old_db_url {
            env::set_var("DB_URL", db_url);
        } else {
            env::remove_var("DB_URL");
        }
    }
}
