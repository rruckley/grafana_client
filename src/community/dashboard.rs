//! Dashboard Module
//! 
use crate::error::GrafanaError;
use log::{info,error};

/// Dashboard Model
pub struct DashboardModel {
    id : Option<String>,
    uid : Option<String>,
    title : String,
    tags : Option<Vec<String>>,
    timezone : String,
    schema_version : u16,
    refresh : String,
}
/// Data for creating a new dashboard
pub struct CreateDashboard {
    dashboard : DashboardModel,
    folder_id : u16,
    folder_uid: Option<String>,
    message : String,
    overwrite : bool,
}

/// Dashboard API Structure
pub struct Dashboard {}

impl Dashboard {
    /// Create a new dashboard in Grafana
    pub fn create(create : CreateDashboard) -> Result<DashboardModel,GrafanaError> {
        match Dashboard::validate_for_create(create) {
            Ok(_) => info!("Create payload ok"),
            Err(e) => error!("Payload failed validation: {}",e),
        } 
        Err(GrafanaError {
            message : "Not implemented".to_string(),
            status : "ERROR".to_string(),
        })
    }
    fn validate_for_create(create : CreateDashboard) -> Result<(),String> {
        match create.dashboard.id {
            Some(_) => Err("Cannot specifiy id on create".to_string()),
            None => Ok(()),
        }
    }
}