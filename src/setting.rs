use config::{Config, ConfigError, Environment};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Setting {
    pub host: String,
    pub database_url: String,
}

impl Setting {
    pub fn from_env() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(Environment::default())
            .build()?;

        s.try_deserialize()
    }
}
