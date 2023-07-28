//! Data Source Module
//! 
//! Identifier (id) vs unique identifier (uid)
//! The identifier (id) of a dashboard is an auto-incrementing numeric value and is only unique per Grafana install.
//!
//! The unique identifier (uid) of a dashboard can be used for uniquely identify a dashboard between multiple Grafana installs. It’s automatically generated if not provided when creating a dashboard. The uid allows having consistent URLs for accessing dashboards and when syncing dashboards between multiple Grafana installs, see dashboard provisioning for more information. This means that changing the title of a dashboard will not break any bookmarked links to that dashboard.
//!
//! The uid can have a maximum length of 40 characters.
use log::debug;
/// Data Source Structure
pub struct DataSource {}

impl DataSource {
    /// Create a new dashboard
    pub fn create(body : String) -> Result<(),String> {
        debug!("BODY: {}",body);
        Err("Not implemented".to_string())
    } 
}