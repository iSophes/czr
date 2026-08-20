use config::{Config, Value};
use std::{collections::HashMap, error, process::exit, vec};

use crate::util::util::display_error;

#[derive(Default)]
pub struct Settings {
    pub enable_emojis: bool,
    pub enable_commit_names: bool,
    pub commit_types: vec::Vec<Value>,
    pub commit_emojis: HashMap<String, Value>,
    pub commit_descriptions: HashMap<String, Value>,
    pub commit_codes: HashMap<String, Value>,
}

impl Settings {
    pub fn new() -> Self {
        return Self::default();
    }

    pub fn setup(&mut self) {
        let settings = Config::builder()
            .add_source(config::File::with_name("config"))
            .add_source(config::Environment::with_prefix("APP"))
            .build()
            .unwrap();

        let deserialised = settings
            .try_deserialize::<HashMap<String, config::Value>>()
            .expect("Failed to load config.");

        // this codes gonna be ass. ill try and figure out a refactor at some point

        for x in deserialised.into_iter() {
            match x.0.as_str() {
                "enable_emojis" => {
                    self.enable_emojis = x.1.into_bool().unwrap();
                }

                "enable_commit_names" => {
                    self.enable_commit_names = x.1.into_bool().unwrap();
                }

                "commit_types" => {
                    self.commit_types = x.1.into_array().unwrap();
                }

                "commit_emojis" => {
                    self.commit_emojis = x.1.into_table().unwrap();
                }

                "commit_descriptions" => {
                    self.commit_descriptions = x.1.into_table().unwrap();
                }

                "commit_codes" => {
                    self.commit_codes = x.1.into_table().unwrap();
                }

                _ => {
                    display_error(&format!(
                        "Config {conf} is missing from the code.",
                        conf = x.0
                    ));
                    exit(1);
                }
            }
        }
    }
}
