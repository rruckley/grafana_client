//! Search Module

use crate::error::GrafanaError;

/// Dashboard Search Results
pub struct DashboardResult {
    id  : String,
}

/// Folder Search Results
pub struct FolderResult {
    id  : String,
}

/// Search Structure
pub struct Search {}

impl Search {
    /// Search dashboards according to query string
    pub fn dashboard(query : Option<String>) -> Result<Vec<DashboardResult>,GrafanaError> {
        Err(GrafanaError { message: String::from("Not implemented"), status: String::from("-1") })
    }

    pub fn folder(query : Option<String>) -> Result<Vec<FolderResult>,GrafanaError> {
        Err(GrafanaError { message: String::from("Not implemented"), status: String::from("-1") })
    }
}