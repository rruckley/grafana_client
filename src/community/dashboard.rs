//! Dashboard Module
//! 
use crate::error::GrafanaError;
use log::{info,error};

/// Panel Model
pub struct PanelModel {

}

/// Complete Dashboard Model
pub struct DashboardModel {
    id : Option<String>,
    uid : Option<String>,
    panels : Option<Vec<PanelModel>>,
    title : String,
    tags : Option<Vec<String>>,
    timezone : String,
    schema_version : u16,
    refresh : String,
}


/// Dashboard API Structure
pub struct Dashboard {
    /// The complete dashboard model, id = null to create a new dashboard.
    dashboard : DashboardModel,
    folder_id : Option<u16>,
    folder_uid: Option<String>,
    message : Option<String>,
    overwrite: bool,
}

impl Dashboard {
    /// Create a new dashboard in Grafana
    /// # Examples
    /// ```
    /// let builder = DashboardBuilder("My Dashboard").build().create("New Dashboard");
    /// ```
    pub fn create(mut self,message : String) -> Result<DashboardModel,GrafanaError> {
        self.message = Some(message);
        match Dashboard::validate_for_create(self) {
            Ok(_) => info!("Create payload ok"),
            Err(e) => error!("Payload failed validation: {}",e),
        } 
        Err(GrafanaError {
            message : "Not implemented".to_string(),
            status : "ERROR".to_string(),
        })
    }
    fn validate_for_create(dashboard : Dashboard) -> Result<(),String> {
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
    panels : Option<Vec<PanelModel>>,
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
            panels : None,
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

    /// Add panel models
    /// # Examples
    /// ```
    /// let some_panels = vec![PanelBuilder::new("my panel").build()];
    /// let dashboard = DashboardBuilder("My Dashboard")
    ///     .panels(some_panels)
    ///     .build()
    ///     .create("New Dashboard");
    /// ```
    pub fn panels(mut self, panels : Vec<PanelModel>) -> DashboardBuilder {
        self.panels = Some(panels);
        self
    }

    /// Build the Dashboard
    pub fn build(self) -> Dashboard {
        let model = DashboardModel {
            id : self.id,
            uid : self.uid,
            panels : None,
            title : self.title,
            tags : None,
            timezone : self.timezone.unwrap_or_default(),
            schema_version : 0,
            refresh : self.refresh.unwrap_or_default(),
        };
        Dashboard {
            dashboard : model,
            folder_id : Some(0),
            folder_uid : None,
            message : None,
            overwrite : false,
        }
    }
}