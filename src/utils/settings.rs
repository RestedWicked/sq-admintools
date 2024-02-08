use serde::Deserialize;

use std::{fs::File, io::Read};

fn default_ip() -> String {
    "0.0.0.0".to_string()
}

fn default_port() -> u32 {
    3000
}

#[derive(Deserialize, Clone)]
pub struct Settings {
    #[serde(default = "default_ip")]
    pub ip: String,

    #[serde(default = "default_port")]
    pub port: u32,
}

impl Settings {
    pub fn new() -> Option<Self> {
        Self::from_file("config.yml")
    }

    pub fn from_file(path: &str) -> Option<Self> {
        let mut settings_yaml = String::new();
        if let Ok(mut file) = File::open(path) {
            file.read_to_string(&mut settings_yaml).ok()?;
        }

        Self::from_str(&settings_yaml)
    }

    pub fn from_str(yaml: &str) -> Option<Self> {
        serde_yaml::from_str(yaml).ok()
    }
}
