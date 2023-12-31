//! Grafana Client
//! 

use crate::common::config::Config;
use crate::common::api::Api;

use crate::community::admin::Admin;
use crate::community::annotations::Annotations;
use crate::community::alerting_provisioning::AlertingProvisioning;
use crate::community::authentication::Authentication;
use crate::community::dashboard::Dashboard;
use crate::community::data_source::DataSource;
use crate::community::folder::Folder;
use crate::community::organization::Organization;
use crate::community::search::Search;

/// Client Structure
pub struct Client {
    /// API Instance'
    pub api : Api,
    /// Common configuration information
    pub config : Config,
    /// Admin API
    pub admin : Admin,
    /// Annoations API
    annotations : Option<Annotations>,
    /// Alert Provsioning API
    alerting_provisioning : Option<AlertingProvisioning>,
    /// Authentication API
    pub authentication : Authentication,
    /// Folder API
    folder : Option<Folder>,
    /// Dashboard API
    dashboard : Option<Dashboard>,
    /// Search API
    search : Option<Search>,
    /// Data Source API
    data_source: Option<DataSource>,
    /// Organization API
    organization : Option<Organization>,
}

impl Client {
    /// Create a new client instance
    /// # Example
    /// ```
    /// # use grafana_lib::client::Client;
    /// let client = Client::new(String::from("http://localhost:3000/"));
    /// ```
    pub fn new(url : String) -> Client {
        let config = Config::new(url.clone())
            .with_token(Config::get_env("GRAFANA_TOKEN").unwrap_or(String::from("DUMMYTOKEN")));
        let api = Api::new(url.clone(),config.get("GRAFANA_TOKEN").unwrap());
        Client {
            api,
            config,
            admin : Admin {},
            annotations : None,
            alerting_provisioning : None,
            authentication : Authentication {  },
            dashboard : None,
            folder : None,
            search : None,
            data_source : None,
            organization : None,
        }
    }

    /// Access instance of Annotations API
    /// 
    /// # Example
    /// ```
    /// # use grafana_lib::client::Client;
    /// # let client = Client::new(String::from("http://localhost:3000/"));
    /// let ap = client.annotations();
    /// ```
    pub fn annotations(mut self) -> Annotations {
        match self.annotations {
            Some(a) => a,
            None => {
                self.annotations = Some(Annotations::new(self.api));
                self.annotations.unwrap()
            }
        }
    }

    /// Create new instance of Alert Provisioning API
    /// # Example
    /// ```
    /// # use grafana_lib::client::Client;
    /// # let client = Client::new(String::from("http://localhost:3000/"));
    /// let ap = client.alerting_provisioning();
    /// ```
    pub fn alerting_provisioning(mut self) -> AlertingProvisioning {
        match self.alerting_provisioning {
            Some(ap) => ap,
            None    => {
                self.alerting_provisioning = Some(AlertingProvisioning::new(self.api));
                self.alerting_provisioning.unwrap()
            }
        }
    }

    /// Return an instance of Dashboard API
    pub fn dashboard(mut self) -> Dashboard {
        match self.dashboard {            None => {
                self.dashboard = Some(Dashboard::new(self.api));
                self.dashboard.unwrap()
            }
            Some(d) => d,

        }
    }

    /// Access instance of DataSource API
    /// # Example
    /// ```
    /// # use grafana_lib::client::Client;
    /// # let client = Client::new(String::from("http://localhost:3000"));
    /// let ds = client.data_source();
    /// ```
    pub fn data_source(mut self) -> DataSource {
        match self.data_source {
            Some(ds) => ds,
            None => {
                self.data_source = Some(DataSource::new(
                    // Safe to simply unwrap here as we would have paniced earlier if these weren't defined
                    self.config.get("GRAFANA_HOST").unwrap(),
                    self.config.get("GRAFANA_TOKEN").unwrap(),
                ));
                self.data_source.unwrap()
            }
        }
    }

    /// Create new instance of Folder API
    pub fn folder(mut self) -> Folder {
        match self.folder {
            Some(f) => f,
            None => {
                self.folder = Some(Folder::new());
                self.folder.unwrap()
            }
        }
    }

    /// Create new instance of Organization API
    pub fn organization(mut self) -> Organization {
        match self.organization {
            Some(o) => o,
            None =>  {
                self.organization = Some(Organization::new());
                self.organization.unwrap()
            }
        }
    }

    /// Create new instance of Search API
    pub fn search(mut self) -> Search {
        match self.search {
            Some(s) => s,
            None => {
                self.search = Some(Search::new(self.api));
                self.search.unwrap()
            }
        }
    }
}