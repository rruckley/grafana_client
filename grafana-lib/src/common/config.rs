//! Config data management
//! 

use std::env;

/// Handles configuration data
#[derive(Debug)]
pub struct Config {
    /// Private field
    host : String,
    token : Option<String>,
}

impl Config {
    /// Create a new configuration instance
    pub fn new(host : String) -> Config {
        Config {
            host,
            token : None,
        }
    }

    /// Add token into config object
    pub fn with_token(mut self, token : String) -> Config {
        self.token = Some(token);
        self
    }

    /// Return URL for connecting to Grafana
    pub fn url(&self) -> String {
        format!("https://{}",self.host).to_string()
    }

    /// Get a single configuration first from ENV then falling back to hard coded defaults
    pub fn get(&self, item : & str) -> Option<String> {
        if item == "GRAFANA_HOST" {
            return Some(self.host.clone());
        }
        if item == "GRAFANA_TOKEN" {
            return self.token.clone()
        }
        match Config::get_env(item) {
            Some(i) => Some(i),
            _ => Config::get_default(item),
        }
    }
    /// Get a configuration from the environment
    pub fn get_env(item : &str) -> Option<String> {
        match env::var(item) {
            Ok(r) => Some(r),
            Err(_) => None,
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