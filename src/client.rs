//! Grafana Client 
//! 

use community::*;
use config;

/// Client Structure
pub struct Client {
    // Commoon
    pub config : Config,
    // Community
    pub admin : Admin,
    pub annotations : Annotations,
    pub alerting_provisioning : AlertingProvisioning,
}


impl Client {
    pub fn new() -> Client {
        Client {

        }
    }
}