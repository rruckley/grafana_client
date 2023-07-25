//! Grafana Client 
//! 

use crate::error::GrafanaError;

use crate::config::Config;
use crate::community::admin::Admin;
use crate::community::annotations::Annotations;
use crate::community::alerting_provisioning::AlertingProvisioning;
use crate::community::authentication::Authentication;

/// Client Structure
pub struct Client {
    /// Common configuration information
    pub config : Config,
    /// Admin API
    pub admin : Admin,
    /// Annoations API
    pub annotations : Annotations,
    /// Alert Provsioning API
    pub alerting_provisioning : AlertingProvisioning,
    /// Authentication API
    pub authentication : Authentication,
}


impl Client {
    /// Create a new client instance
    pub fn new(url : String) -> Client {
        Client {
            config : Config::new(url),
            admin : Admin {},
            annotations : Annotations {  },
            alerting_provisioning : AlertingProvisioning {  },
            authentication : Authentication {  },
        }
    }

    pub fn connect() -> Result<(),GrafanaError> {
        Ok(())
    }
}