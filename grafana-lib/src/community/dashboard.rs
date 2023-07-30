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
    /// use grafana_lib::community::dashboard::{DashboardBuilder,PanelBuilder};
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
    pub fn build(self) -> Dashboard {
        let model = DashboardModel {
            id : self.id,
            uid : self.uid,
            panels : None,
            title : self.title,
            tags : None,
            timezone : self.timezone.unwrap_or_default(),
            schema_version : self.schema_version,
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


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_new_dashboard() {
        let dashboard = DashboardBuilder::new("test".to_string()).build();
        let test_dashboard = Dashboard {
            dashboard : DashboardModel {
                id : None,
                panels : None,
                title : String::from("test"),
                uid : None,
                timezone : String::from(""),
                schema_version : 0,
                tags : None,
                refresh : String::from(""),
            },
            overwrite : false,
            folder_id  : Some(0),
            folder_uid : None,
            message : None,
        };
        assert_eq!(dashboard,test_dashboard);
    }
    #[test]
    fn test_dashboard_with_schema_version() {
        let dashboard = DashboardBuilder::new("test".to_string())
            .schema_version(1)
            .build();
        let test_dashboard = Dashboard {
            dashboard : DashboardModel {
                id : None,
                panels : None,
                title : String::from("test"),
                uid : None,
                timezone : String::from(""),
                schema_version : 1,
                tags : None,
                refresh : String::from(""),
            },
            overwrite : false,
            folder_id  : Some(0),
            folder_uid : None,
            message : None,
        };
        assert_eq!(dashboard,test_dashboard);
    }

}