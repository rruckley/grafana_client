//! Grafana Client
//! 

use crate::community::admin::Admin;
use crate::community::annotations::Annotations;
use crate::community::alerting_provisioning::AlertingProvisioning;
use crate::community::authentication::Authentication;
use crate::community::dashboard::Dashboard;
use crate::community::search::Search;

use crate::common::config::Config;
use crate::common::api::Api;

/// Client Structure
pub struct Client {
    /// API Instance'
    pub api : Api,
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
    dashboard : Option<Dashboard>,
    /// Search API
    search : Option<Search>,
}

impl Client {
    /// Create a new client instance
    pub fn new(url : String) -> Client {
        let api = Api::new(url.clone(),Config::get("TOKEN").unwrap_or(String::from("DUMMYTOKEN")));
        Client {
            api,
            config : Config::new(url),
            admin : Admin {},
            annotations : Annotations {  },
            alerting_provisioning : AlertingProvisioning {  },
            authentication : Authentication {  },
            dashboard : None,
            search : None,
        }
    }

    /// Return an instance of Dashboard API
    pub fn dashboard(mut self) -> Dashboard {
        match self.dashboard {
            Some(d) => d,
            None => {
                self.dashboard = Some(Dashboard::new());
                self.dashboard.unwrap()
            }
        }
    }

    /// Create new insetance of Search API
    pub fn search(mut self) -> Search {
        match self.search {
            Some(s) => s,
            None => {
                let host = Config::get("GRAFANA_HOST").expect("GRAFANA_HOST not found");
                let token = Config::get("GRAFANA_TOKEN").expect("GRAFANA_TOKEN not found");
                self.search = Some(Search::new(host,token));
                self.search.unwrap()
            }
        }
    }

    /// Search Folders
    pub fn folders(mut self, query : Option<String>) -> String {
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
    pub fn connect() -> Result<String,String> {
        Ok(String::from("Wow!"))
    }
}