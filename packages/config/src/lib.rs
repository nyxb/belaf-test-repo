//! Configuration management for the belaf test monorepo

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to parse config: {0}")]
    ParseError(String),
    #[error("Missing required field: {0}")]
    MissingField(String),
}

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub name: String,
    pub version: String,
    pub debug: bool,
    pub port: u16,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            name: "app".to_string(),
            version: "0.1.0".to_string(),
            debug: false,
            port: 3000,
        }
    }
}

impl AppConfig {
    /// Create a new config with the given name
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }

    /// Load config from JSON string
    pub fn from_json(json: &str) -> Result<Self, ConfigError> {
        serde_json::from_str(json)
            .map_err(|e| ConfigError::ParseError(e.to_string()))
    }

    /// Serialize config to JSON string
    pub fn to_json(&self) -> Result<String, ConfigError> {
        serde_json::to_string_pretty(self)
            .map_err(|e| ConfigError::ParseError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.port, 3000);
        assert!(!config.debug);
    }

    #[test]
    fn test_config_json_roundtrip() {
        let config = AppConfig::new("test-app");
        let json = config.to_json().unwrap();
        let parsed = AppConfig::from_json(&json).unwrap();
        assert_eq!(config.name, parsed.name);
    }
}
