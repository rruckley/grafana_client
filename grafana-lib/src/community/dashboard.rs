//! Dashboard Module
//! 
use crate::common::error::GrafanaError;
use crate::common::api::Api;

use serde::Deserialize;
use std::fmt;

const DASHBOARD_PATH : &str = "dashboards";
const DASHBOARD_UID_PATH : &str = "uid";

/// Panel Model
#[derive(PartialEq,Debug,Deserialize)]
pub struct PanelModel {
    title   : String,
    id  : u16,
    r#type : String,
}

impl fmt::Display for PanelModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::default();
        output.push_str(format!("Title\t: {} [{}] - {}",
            self.title.clone(),
            self.id,
            self.r#type,
        ).as_str());
        writeln!(f, "{output}")
    }
}

/// Complete Dashboard Model
#[derive(PartialEq,Debug,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DashboardModel {
    id : Option<u32>,
    /// Unique Id of Dashboard, used in other queries
    pub uid : Option<String>,
    /// Vector of panels
    pub panels : Option<Vec<PanelModel>>,
    /// Title of dashboard
    pub title : Option<String>,
    tags : Option<Vec<String>>,
    timezone : Option<String>,
    /// Schema Version
    pub schema_version : Option<u16>,
    refresh : Option<String>,
    /// Dashboard Version
    pub version : Option<u16>,
}

impl fmt::Display for DashboardModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::default();
        output.push_str(format!("Title\t: {}\n",self.title.clone().unwrap_or_default()).as_str());
        output.push_str(format!("UID\t: {}\n",self.uid.clone().unwrap()).as_str());
        output.push_str(format!("Version\t: {}\n",self.version.unwrap()).as_str());
        output.push_str(format!("Schema\t: {}\n",self.schema_version.unwrap()).as_str());
        write!(f, "{output}" )
    }
}

/// Dashboard meta-data
#[derive(Debug,PartialEq,Default,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaModel {
    /// Folder Name
    pub folder_title : Option<String>,
    /// Original creator of the dashboard
    pub created_by  : Option<String>,
    /// Last updated by
    pub updated_by  : Option<String>,
}

impl fmt::Display for MetaModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::default();
        output.push_str(format!("Folder\t: {}\n",self.folder_title.clone().unwrap_or_default()).as_str());
        output.push_str(format!("Created\t: {}\n",self.created_by.clone().unwrap_or_default()).as_str());
        output.push_str(format!("Updated\t: {}\n",self.updated_by.clone().unwrap_or_default()).as_str());
        write!(f, "{output}" )
    }
}

/// Full Dashboard Model
#[derive(Debug,PartialEq,Deserialize)]
pub struct FullDashboardModel {
    /// Meta-data on dashboard
    pub meta : MetaModel,
    /// Dashboard model
    pub dashboard : DashboardModel,
}


/// Dashboard API Structure
#[derive(PartialEq,Debug,Default)]
pub struct Dashboard {
    /// API interface
    api : Api,
    /// The complete dashboard model, id = null to create a new dashboard.
    dashboard : Option<DashboardModel>,
    folder_id : Option<u16>,
    folder_uid: Option<String>,
    message : Option<String>,
    overwrite: bool,
}

impl Dashboard {
    /// Create a new instance of Dashboard API
    pub fn new(api : Api) -> Dashboard {
        Dashboard { api, dashboard: None, folder_id: None, folder_uid: None, message: None, overwrite: false }
    }

    /// Create a new dashboard in Grafana
    /// # Examples
    /// ```
    /// # use grafana_lib::community::dashboard::{Dashboard,DashboardBuilder,PanelBuilder};
    /// # use grafana_lib::common::api::Api;
    /// # let api = Api::default();
    /// # let dashboard = Dashboard::new(api);
    /// let model = DashboardBuilder::new(String::from("My Dashboard"))
    ///     .build();
    /// dashboard
    ///     .create(model)
    ///     .with_message(String::from("My New Dashboard"))
    ///     .send();
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

    /// Get a dashboard by UID
    pub fn get(&self, uid : String) -> Result<FullDashboardModel,GrafanaError> {
        let path = format!("{}/{}/{}",DASHBOARD_PATH,DASHBOARD_UID_PATH,uid);
        match self.api.get(path) {
            Ok(r) => {
                let result : FullDashboardModel = serde_json::from_str(r.as_str()).expect("Could not parse JSON");
                Ok(result)
            },
            Err(e) => {
                Err(GrafanaError::new(e,String::from("ERROR")))
            }
        }
        
    }
}

/// Builder for Panels
pub struct PanelBuilder {
    title : String,
    r#type : String,
}

impl PanelBuilder {
    /// Create a new PanelBuilder object
    pub fn new(title : String) -> PanelBuilder {
        PanelBuilder { title, r#type : String::default() } 
    }
    /// Set type of panel
    pub fn with_type(mut self, r#type : String) -> PanelBuilder {
        self.r#type = r#type;
        self
    }
    /// Build a Panel
    pub fn build(self) -> PanelModel {
        PanelModel { title : self.title, id : 0, r#type : self.r#type}
    }
}

/// Builder for Dashboard
#[derive(PartialEq,Debug)]
pub struct DashboardBuilder {
    id : Option<u32>,
    uid : Option<String>,
    panels : Option<Vec<PanelModel>>,
    title : String,
    tags : Option<Vec<String>>,
    timezone : Option<String>,
    schema_version : u16,
    version : u16,
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
            version : 0,
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
    /// # use grafana_lib::community::dashboard::{DashboardBuilder,PanelBuilder};
    /// # use grafana_lib::client::Client;
    /// # let client = Client::new(String::from("http://localhost:3000"));
    /// let panel = PanelBuilder::new(String::from("MyPanel"))
    ///     .build();
    /// let panel_vec = vec![panel];
    /// let model = DashboardBuilder::new(String::from("MyDashboard"))
    ///     .with_panels(panel_vec)
    ///     .build();
    /// let _output = client.dashboard()
    ///     .create(model)
    ///     .with_message(String::from("New Dashboard via CLI"))
    ///     .with_folder_id(6)
    ///     .send();
    /// ```
    pub fn with_panels(mut self, panels : Vec<PanelModel>) -> DashboardBuilder {
        self.panels = Some(panels);
        self
    }

    /// Build the Dashboard
    /// # Examples
    /// ```
    /// # use grafana_lib::community::dashboard::DashboardBuilder;
    /// # use grafana_lib::client::Client;
    /// let model = DashboardBuilder::new(String::from("MyDashboard"))
    ///     .build();
    /// ```
    pub fn build(self) -> DashboardModel {
        DashboardModel {
            id : self.id,
            uid : self.uid,
            panels : None,
            title : Some(self.title),
            tags : None,
            timezone : self.timezone,
            schema_version : Some(self.schema_version),
            refresh : self.refresh,
            version : Some(self.version),
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
            title : Some(String::from("test")),
            uid : None,
            timezone : None,
            schema_version : Some(0),
            tags : None,
            refresh : None,
            version : Some(0),
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
            title : Some(String::from("test")),
            uid : None,
            timezone : None,
            schema_version : Some(1),
            tags : None,
            refresh : None,
            version: Some(0),
        };
        assert_eq!(dashboard,test_dashboard);
    }

}