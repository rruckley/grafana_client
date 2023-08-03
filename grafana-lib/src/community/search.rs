//! Search Module

use crate::common::error::GrafanaError;
use crate::common::api::Api;

use log::debug;

const SEARCH_PATH       : &str = "search";
const SEARCH_DASHBOARD  : &str = "dash-db";
const SEARCH_FOLDER     : &str = "dash-folder";

/// Dashboard Search Results
pub struct DashboardResult {
    _id  : String,
}

/// Folder Search Results
pub struct FolderResult {
    _id  : String,
}

/// Search Structure
pub struct Search {
    api : Api,
}

impl Search {
    /// Create a new search instance
    pub fn new(host : String, token : String) -> Search {
        let api = Api::new(host,token);
        Search {
            api,
        }
    }
    /// Search dashboards according to query string
    pub fn dashboard(&self, query : Option<String>) -> Result<Vec<DashboardResult>,GrafanaError> {
        let url = match query {
            Some(q) => format!("{}?type={}&query={}",SEARCH_PATH,SEARCH_DASHBOARD,q),
            None    => format!("{}?type={}",SEARCH_PATH,SEARCH_DASHBOARD),
        };
        debug!("URL: {url}");
        let _json = self.api.get(url);
        Err(GrafanaError { message: String::from("Dashboard Search: Not implemented"), status: String::from("-1") })
    }

    /// Folder Search using query string
    pub fn folder(&self, query : Option<String>) -> Result<Vec<FolderResult>,GrafanaError> {
        let url = match query {
            Some(q) => format!("{}?type={}&query={}",SEARCH_PATH,SEARCH_FOLDER,q),
            None    => format!("{}?type={}",SEARCH_PATH,SEARCH_FOLDER),
        };
        debug!("URL: {url}");
        let _json = self.api.get(url);
        Err(GrafanaError { message: String::from("Folder Search: Not implemented"), status: String::from("-1") })
    }
}