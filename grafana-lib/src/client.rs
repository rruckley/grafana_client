//! Grafana Client
//! 
use crate::config::Config;
use crate::community::admin::Admin;
use crate::community::annotations::Annotations;
use crate::community::alerting_provisioning::AlertingProvisioning;
use crate::community::authentication::Authentication;
use crate::community::dashboard::Dashboard;

use serde::Serialize;
use reqwest::*;

/// Client Structure
pub struct Client {
    /// Common configuration information
    pub config : Config,
    /// Admin API
    pub admin : Admin,
    /// Annoations API
    pub annotations : Annotations,
    /// Alert Provsioning API
    pub alerting_provisioning : AlertingProvisioning,
    /// Authentication API
    pub authentication : Authentication,
    /// Dashboard API
    pub dashboard : Option<Dashboard>,
}

impl Client {
    /// Create a new client instance
    pub fn new(url : String) -> Client {
        Client {
            config : Config::new(url),
            admin : Admin {},
            annotations : Annotations {  },
            alerting_provisioning : AlertingProvisioning {  },
            authentication : Authentication {  },
            dashboard : None,
        }
    }

    pub fn search_dashboards(&self, query : Option<String>) -> String {
        String::from("Not implemented")
    }

    pub fn search_folders(&self, query : Option<String>) -> String {
        String::from("Folder Search: Not implemented")
    }

    /// Send compatible struct through to Grafana
    async fn call<T>(&self, payload : T) -> Result<String> 
    where T : Sized + Serialize,
    {
        let client = reqwest::Client::new();
        match client.post(self.config.url())
            .json(&payload)
            .send().await {
                Ok(_) => Ok("Yay".to_string()),
                Err(e) => Err(e),
            }
    }

    /// Connect to Grafana
    pub fn connect() -> Result<String> {
        Ok(String::from("Wow!"))
    }
}