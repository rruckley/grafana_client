//! Config data management
//! 

use std::env;

/// Handles configuration data
pub struct Config {
    /// Private field
    host : String,
}

impl Config {
    /// Create a new configuration instance
    pub fn new(host : String) -> Config {
        Config {
            host
        }
    }
    /// Return URL for connecting to Grafana
    pub fn url(&self) -> String {
        format!("https://{}",self.host).to_string()
    }
    /// Get a single configuration first from ENV then falling back to hard coded defaults
    pub fn get(item : & str) -> Option<String> {
        match env::var(item) {
            Ok(i) => Some(i),
            _ => Config::get_default(item),
        }
    }

    /// Return hard coded default configuration items
    fn get_default(item : &str) -> Option<String> {
        match item {
            "VERSION" => {
                let name = env!("CARGO_PKG_NAME");
                let vers = env!("CARGO_PKG_VERSION");
                Some(format!("{}/{}",name,vers))
            },
            _ => None,
        }
    }
}