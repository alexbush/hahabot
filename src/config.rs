use serde::Deserialize;
use std::io::Read;

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    pub token: Option<String>,
    pub chats: Option<Vec<i64>>,
}

impl Config {
    pub fn from_path(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
        let mut file    = std::fs::File::open(path)?;
        let mut content = String::new();

        file.read_to_string(&mut content)?;

        let config      = toml::de::from_str::<Self>(&content)?;
        Ok(config)
    }
}
