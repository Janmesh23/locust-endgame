use serde::Deserialize;
use serde_json;
use std::fs; // Make sure serde_json is explicitly imported

#[derive(Debug, Deserialize)]
pub struct Config {
    pub interval: u64,
    pub log_path: String,
    pub api_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            interval: 60, // 1 minute in seconds
            log_path: "locations.jsonl".into(),
            api_url: "http://ip-api.com/json".into(),
        }
    }
}

pub fn load_config() -> Config {
    fs::read_to_string("config.json")
        .ok() // Convert Result to Option, so we get None if file not found
        .and_then(|s| serde_json::from_str(&s).ok()) // Try to deserialize, get None if parsing fails
        .unwrap_or_default() // If either is None, use the default Config
}

pub fn print_config() {
    let cfg = load_config();
    println!("{:#?}", cfg);
}
