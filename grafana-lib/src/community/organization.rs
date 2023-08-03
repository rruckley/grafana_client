//! Organization Module
//! 
//! This module handles management of organizations within Grafana


/// Organization Struct
#[derive(Debug,Default)]
pub struct Organization {
    name : Option<String>,
    user : Option<String>,
    pass : Option<String>,
}

impl Organization {
    /// Create empty instance of organization
    pub fn new() -> Organization {
        Organization { 
            name : None,
            user : None,
            pass : None, 
        }
    }
    /// Create a new organisation model
    /// # Examples
    /// ```
    /// # use grafana_lib::client::Client;
    /// # let client = Client::new(String::from("http://localhost:3000"));
    /// let result = client.organization()
    ///     .create(String::from("MyOrg"))
    ///     .send();
    ///    
    /// ```
    pub fn create(mut self,name : String) -> Organization {
        self.name = Some(name);
        self
    }
    /// Set User for Basic Authentication
    pub fn with_user(mut self, user : String) -> Organization {
        self.user = Some(user);
        self
    }
    /// Set Password for Basic Authentication
    pub fn with_pass(mut self, pass : String) -> Organization {
        self.pass = Some(pass);
        self
    }
    /// Send organization to Grafana
    pub fn send(&self) -> Result<String,String> {
        if self.user.is_none() || self.pass.is_none() {
            return Err(String::from("Basic Auth Credentials not set"));
        }
        Err(String::from("Org Create: Not implemented"))
    }
}