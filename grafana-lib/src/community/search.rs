//! Search Module

use crate::common::error::GrafanaError;
use crate::common::api::Api;

/// Dashboard Search Results
pub struct DashboardResult {
    id  : String,
}

/// Folder Search Results
pub struct FolderResult {
    id  : String,
}

/// Search Structure
pub struct Search {
    api : Api,
}

impl Search {
    /// Create a new search instance
    pub fn new(host : String, token : String) -> Search {
        let api = Api::new(token,host);
        Search {
            api,
        }
    }
    /// Search dashboards according to query string
    pub fn dashboard(&self, query : Option<String>) -> Result<Vec<DashboardResult>,GrafanaError> {
        let url = format!("{}?{}",self.api.host,query.unwrap());
        let json = self.api.get(url);
        Err(GrafanaError { message: String::from("Dashboard Search: Not implemented"), status: String::from("-1") })
    }

    /// Folder Search using query string
    pub fn folder(&self, query : Option<String>) -> Result<Vec<FolderResult>,GrafanaError> {
        Err(GrafanaError { message: String::from("Folder Search: Not implemented"), status: String::from("-1") })
    }
}