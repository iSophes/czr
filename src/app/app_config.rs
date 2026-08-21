use config::{Config, ConfigError};
use dialoguer::{FuzzySelect, Select};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, process::exit};

use crate::util::util::display_error;

const DEFAULT_CONFIG: &str = include_str!("../../config.toml");

#[derive(Default, Deserialize, Serialize)]
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

        if !self.enable_commit_names && !self.enable_emojis {
            display_error(
                "Commit names and emojis are both disabled, only one or the other can be disabled. Defaulting to just commit names.",
            );
            self.enable_commit_names = true;
        }

        Ok(())
    }

    pub fn call_settings(&mut self) -> Result<(), ConfigError> {
        // implementing bools for now, as its kinda difficult and out of scope to get commit types and stuff editing atm.

        let configs_to_edit = vec!["enable_emojis", "enable_commit_names"];

        let selection = FuzzySelect::new()
            .with_prompt("What do you choose?")
            .items(&configs_to_edit)
            .interact()
            .unwrap();

        let current_value = match configs_to_edit[selection] {
            "enable_emojis" => self.enable_emojis,
            "enable_commit_names" => self.enable_commit_names,
            _ => {
                // this shouldnt happen.
                display_error(
                    "You've managed to edit a non-existent config. Please report this as an issue on github. Aborting...",
                );
                exit(1);
            }
        };

        let selected_value = Select::new()
            .with_prompt(format!(
                "Editing {selection}. Current value: {x}",
                selection = selection,
                x = current_value
            ))
            .items([true, false])
            .default(if current_value { 0 } else { 1 })
            .interact()
            .unwrap();

        let selected_boolean = [true, false][selected_value];

        match configs_to_edit[selection] {
            "enable_emojis" => self.enable_emojis = selected_boolean,
            "enable_commit_names" => self.enable_commit_names = selected_boolean,
            _ => {
                // this shouldnt happen.
                display_error(
                    "You've managed to edit a non-existent config. Please report this as an issue on github. Aborting...",
                );
                exit(1);
            }
        };

        let config_path = dirs::config_dir().unwrap().join("czr/config.toml");
        std::fs::create_dir_all(config_path.parent().unwrap())
            .map_err(|_| ConfigError::Message("Failed to create config directory".into()))?;
        let toml_string = toml::to_string_pretty(&self)
            .map_err(|_| ConfigError::Message("Failed to serialize config".into()))?;
        std::fs::write(&config_path, toml_string)
            .map_err(|_| ConfigError::Message("Failed to write config file".into()))?;

        Ok(())
    }
}
