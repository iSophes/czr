use config::Config;
use std::{collections::HashMap, vec};

#[derive(Default)]
pub struct Settings {
    enable_emojis: bool,
    commit_types: vec::Vec<String>,
    commit_emojis: HashMap<String, String>,
    commit_descriptions: HashMap<String, String>,
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

        println!("{:?}", self.enable_emojis);

        for x in deserialised.into_iter() {
            if x.0 == "enable_emojis" {
                self.enable_emojis = x.1.into_bool().unwrap();
                println!("{:?}", self.enable_emojis);
            }
        }
    }
}
