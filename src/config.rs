use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct AppConfig {
    pub connection_string: String,
}

impl AppConfig {
    pub fn load() -> Self {
        fs::read_to_string("config.json")
            .ok()
            .and_then(|d| serde_json::from_str(&d).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write("config.json", json);
        }
    }

    pub fn db_connection_string(&self) -> &str {
        &self.connection_string
    }
}