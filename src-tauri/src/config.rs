use std::fs::File;
use std::io::{Read, Write};
use serde_json;
use crate::AppConfig;

pub fn load_config() -> Result<AppConfig, String> {
    let mut file = File::open("config.json").map_err(|e| e.to_string())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| e.to_string())?;
    serde_json::from_str(&contents).map_err(|e| e.to_string())
}

pub fn save_config(config: &AppConfig) -> Result<(), String> {
    let json = serde_json::to_string_pretty(config).map_err(|e| e.to_string())?;
    let mut file = File::create("config.json").map_err(|e| e.to_string())?;
    file.write_all(json.as_bytes()).map_err(|e| e.to_string())
}
