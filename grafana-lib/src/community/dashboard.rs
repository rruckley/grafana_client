//! Dashboard Module
//! 
use crate::common::error::GrafanaError;
use log::{info,error};

/// Panel Model
#[derive(PartialEq,Debug)]
pub struct PanelModel {
    title   : String,
}

/// Complete Dashboard Model
#[derive(PartialEq,Debug)]
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
#[derive(PartialEq,Debug)]
pub struct Dashboard {
    /// The complete dashboard model, id = null to create a new dashboard.
    dashboard : Option<DashboardModel>,
    folder_id : Option<u16>,
    folder_uid: Option<String>,
    message : Option<String>,
    overwrite: bool,
}

impl Dashboard {
    /// Create a new instance of Dashboard API
    pub fn new() -> Dashboard {
        Dashboard { dashboard: None, folder_id: None, folder_uid: None, message: None, overwrite: false }
    }

    /// Create a new dashboard in Grafana
    /// # Examples
    /// ```
    /// use grafana_lib::community::dashboard::{DashboardBuilder,PanelBuilder};
    /// let builder = DashboardBuilder("My Dashboard").build().create("New Dashboard");
    /// ```
    pub fn create(mut self,dashboard : DashboardModel) -> Dashboard {
        self.dashboard = Some(dashboard);
        self
    }

    /// Add commit message
    pub fn with_message(mut self, message : String) -> Dashboard {
        self.message = Some(message);
        self
    }

    /// Set folder Id
    pub fn with_folder_id(mut self,id : u16) -> Dashboard {
        self.folder_id = Some(id);
        self
    }

    /// Set folder UID
    pub fn with_folder_uid(mut self, uid : String) -> Dashboard {
        self.folder_uid = Some(uid);
        self
    }

    /// Set overwrite flag
    pub fn with_overwrite(mut self, overwrite: bool) -> Dashboard {
        self.overwrite = overwrite;
        self
    }

    /// Send Dashboard to Grafana
    pub fn send(self) -> Result<Dashboard,GrafanaError> {
        // Send current data to Grafana
        
        Ok(self)
    }
}

/// Builder for Panels
pub struct PanelBuilder {
    title : String,
}

impl PanelBuilder {
    /// Create a new PanelBuilder object
    pub fn new(title : String) -> PanelBuilder {
        PanelBuilder { title } 
    }
    /// Build a Panel
    pub fn build(self) -> PanelModel {
        PanelModel { title : self.title, }
    }
}

/// Builder for Dashboard
#[derive(PartialEq,Debug)]
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

    /// Set the Schema Version of this dashboard
    pub fn schema_version(mut self, version : u16) -> DashboardBuilder {
        self.schema_version = version;
        self
    }

    /// Add panel models
    /// # Examples
    /// ```
    /// use grafana_lib::community::dashboard::{DashboardBuilder,PanelBuilder};
    /// let some_panels = vec![PanelBuilder::new(String::from("my panel")).build()];
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
    pub fn build(self) -> DashboardModel {
        DashboardModel {
            id : self.id,
            uid : self.uid,
            panels : None,
            title : self.title,
            tags : None,
            timezone : self.timezone.unwrap_or_default(),
            schema_version : self.schema_version,
            refresh : self.refresh.unwrap_or_default(),
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_new_dashboard() {
        let dashboard = DashboardBuilder::new("test".to_string()).build();
        let test_dashboard = DashboardModel {        
            id : None,
            panels : None,
            title : String::from("test"),
            uid : None,
            timezone : String::from(""),
            schema_version : 0,
            tags : None,
            refresh : String::from(""),
        };
        assert_eq!(dashboard,test_dashboard);
    }
    #[test]
    fn test_dashboard_with_schema_version() {
        let dashboard = DashboardBuilder::new("test".to_string())
            .schema_version(1)
            .build();
        let test_dashboard = DashboardModel {
            id : None,
            panels : None,
            title : String::from("test"),
            uid : None,
            timezone : String::from(""),
            schema_version : 1,
            tags : None,
            refresh : String::from(""),
        };
        assert_eq!(dashboard,test_dashboard);
    }

}