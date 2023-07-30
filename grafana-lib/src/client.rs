//! Grafana Client
//! 
use crate::common::config::Config;
use crate::community::admin::Admin;
use crate::community::annotations::Annotations;
use crate::community::alerting_provisioning::AlertingProvisioning;
use crate::community::authentication::Authentication;
use crate::community::dashboard::Dashboard;
use crate::community::search::Search;

use crate::common::api::Api;

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
    /// Search API
    pub search : Option<Search>,
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
            search : None,
        }
    }

    /// Search dashboards
    pub fn search_dashboards(mut self, query : Option<String>) -> String {
        let search = match self.search {
            Some(s) => s,
            None => {
                // Store instance for next query
                
                self.search = Some( Search::new(Config::get("TOKEN").unwrap(),Config::get("HOST").unwrap()));
                self.search.unwrap()
            },
        };
        match search.dashboard(query) {
            Ok(_v) => {
                String::from("Got some results")
            },
            Err(e) => e.message,
        }
    }

    /// Search Folders
    pub fn search_folders(mut self, query : Option<String>) -> String {
        let search = match self.search {
            Some(s) => s,
            None => {
                // Store instance for next query
                self.search = Some( Search::new(Config::get("TOKEN").unwrap(),Config::get("HOST").unwrap()));
                self.search.unwrap()
            },
        };
        match search.folder(query) {
            Ok(_v) => {
                String::from("Got some results")
            },
            Err(e) => e.message,
        }
    }

    /// Connect to Grafana
    pub fn connect() -> Result<String> {
        Ok(String::from("Wow!"))
    }
}