use config::{Config, ConfigError};
use serde::Deserialize;
use std::collections::HashMap;

const DEFAULT_CONFIG: &str = include_str!("../../config.toml");

#[derive(Default, Deserialize)]
pub struct Settings {
    pub enable_emojis: bool,
    pub enable_commit_names: bool, // will be sorted soon

    pub commit_types: Vec<String>,
    pub commit_emojis: HashMap<String, String>,
    pub commit_descriptions: HashMap<String, String>,
    pub commit_codes: HashMap<String, String>,
}

impl Settings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn setup(&mut self) -> Result<(), ConfigError> {
        let user_config_path = dirs::config_dir().map(|p| p.join("czr/config.toml"));

        let mut builder = Config::builder().add_source(config::File::from_str(
            DEFAULT_CONFIG,
            config::FileFormat::Toml,
        ));

        if let Some(path) = user_config_path {
            builder = builder.add_source(config::File::from(path).required(false));
        }

        builder = builder.add_source(config::Environment::with_prefix("APP"));

        let settings = builder.build()?;
        *self = settings.try_deserialize::<Settings>()?;

        Ok(())
    }
}
