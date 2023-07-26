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


/// Dashboard API Structure
pub struct Dashboard {
    dashboard : DashboardModel,
}

impl Dashboard {
    /// Create a new dashboard in Grafana
    /// # Examples
    /// ```
    /// let builder = DashboardBuilder("My Dashboard").build().create();
    /// ```
    pub fn create(&self) -> Result<DashboardModel,GrafanaError> {
        match Dashboard::validate_for_create(self) {
            Ok(_) => info!("Create payload ok"),
            Err(e) => error!("Payload failed validation: {}",e),
        } 
        Err(GrafanaError {
            message : "Not implemented".to_string(),
            status : "ERROR".to_string(),
        })
    }
    fn validate_for_create(dashboard : &Dashboard) -> Result<(),String> {
        match &dashboard.dashboard.id {
            Some(_i) => Err("Cannot specifiy id on create".to_string()),
            None => Ok(()),
        }
    }
}

/// Builder for Dashboard
pub struct DashboardBuilder {
    id : Option<String>,
    uid : Option<String>,
    title : String,
    tags : Option<Vec<String>>,
    timezone : Option<String>,
    schema_version : u16,
    refresh : Option<String>,
    folder_id : Option<u16>,
    folder_uid: Option<String>,
    message : Option<String>,
    overwrite : bool,
}

impl DashboardBuilder {
    /// Create new builder instance
    pub fn new(title : String) -> DashboardBuilder {
        DashboardBuilder {
            id : None,
            uid : None,
            title,
            tags : None,
            timezone : None,
            schema_version : 0,
            refresh : None,
            folder_id : None,
            folder_uid : None,
            message : None,
            overwrite : true,
        }
    }
    /// Set title of Dashboard
    pub fn title(mut self, title : String) -> DashboardBuilder {
        self.title = title;
        self
    }

    /// Build the Dashboard
    pub fn build(self) -> Dashboard {
        let model = DashboardModel {
            id : self.id,
            uid : self.uid,
            title : self.title,
            tags : None,
            timezone : self.timezone.unwrap_or_default(),
            schema_version : 0,
            refresh : self.refresh.unwrap_or_default(),
        };
        Dashboard {
            dashboard : model,
        }
    }
}