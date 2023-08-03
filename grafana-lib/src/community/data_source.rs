//! Data Source Module
//! 
//! Identifier (id) vs unique identifier (uid)
//! The identifier (id) of a dashboard is an auto-incrementing numeric value and is only unique per Grafana install.
//!
//! The unique identifier (uid) of a dashboard can be used for uniquely identify a dashboard between multiple Grafana installs. It’s automatically generated if not provided when creating a dashboard. The uid allows having consistent URLs for accessing dashboards and when syncing dashboards between multiple Grafana installs, see dashboard provisioning for more information. This means that changing the title of a dashboard will not break any bookmarked links to that dashboard.
//!
//! The uid can have a maximum length of 40 characters.
//use log::debug;

/// Data Source Model
#[derive(Debug,Default)]
pub struct DataSourceModel {
    /// Data Source Name
    pub name    : String,
    /// Data Source Type
    pub r#type    : Option<String>,
    /// Data Source URL
    pub url     : Option<String>,
    /// Use Basic Auth?
    pub basic_auth : bool,
}

/// Builder for Data Source Model
pub struct DataSourceBuilder {
    name    : String,
    r#type  : Option<String>,
    url     : Option<String>,
    basic_auth : bool,
}

impl DataSourceBuilder {
    /// Create new instance of Data Source Builder
    pub fn new(name : String) -> DataSourceBuilder {
        DataSourceBuilder { 
            name,
            r#type : None,
            url     : None,
            basic_auth : false, 
        }
    }
    /// Add URL to DataSource
    /// # Example
    /// ```
    /// # use grafana_lib::community::data_source::DataSourceBuilder;
    /// let datasource = DataSourceBuilder::new(String::from("MyDataSource"))
    ///     .with_url(String::from("http://somedata.com"))
    ///     .build();
    /// ```
    pub fn with_url(mut self, url : String) -> DataSourceBuilder {
        self.url = Some(url);
        self
    }

    /// Build a Data Source Model
    /// # Example
    /// ```
    /// # use grafana_lib::community::data_source::DataSourceBuilder;
    /// let model = DataSourceBuilder::new(String::from("MyDataSource"))
    ///     .build();
    /// ```
    pub fn build(self) -> DataSourceModel {
        DataSourceModel {
            name : self.name,
            r#type : self.r#type,
            url : self.url,
            basic_auth : self.basic_auth,
        }
    }
}

/// Data Source Structure
#[derive(Debug,Default)]
pub struct DataSource {
    model : Option<DataSourceModel>,
}

impl DataSource {
    /// Create new DataSource API instance
    /// # Example
    /// ```
    /// # use grafana_lib::community::data_source::DataSource;
    /// let datasource = DataSource::new();
    /// ```
    pub fn new() -> DataSource {
        DataSource { model : None}
    }
    /// Create a new dashboard, can fail if there is a conflict in the data, e.g. folder_id vs folder_uid
    /// # Example
    /// ```
    /// # use grafana_lib::community::data_source::{DataSource,DataSourceBuilder};
    /// # let datasource = DataSource::new();
    /// let model = DataSourceBuilder::new(String::from("MyDataSource"))
    ///     .build();
    /// let result = datasource
    ///     .create(model);
    /// ```
    pub fn create(mut self, model : DataSourceModel) -> Result<DataSource,String> {
        self.model = Some(model);
        Ok(self)
    }

    /// Send new Data Source to Grafana
    /// # Example 
    /// ```
    /// # use grafana_lib::community::data_source::{DataSource,DataSourceBuilder};
    /// # let datasource = DataSource::new();
    /// let model = DataSourceBuilder::new(String::from("MyDataSource"))
    ///     .build();
    /// let result = datasource
    ///     .create(model)
    ///     .expect("Could not create Dashboard instance")
    ///     .send();
    /// ```
    pub fn send(&self) -> Result<String,String> {
        // Send data source to Grafana
        Err(String::from("datasource.send() - Not implemented"))
    }
}