//! Grafana Client 
//! 

use community::*;
use config;

/// Client Structure
pub struct Client {
    // Commoon
    config : Config,
    // Community
    admin : Admin,
    annotations : Annotations,
}


impl Client {
    pub fn new() -> Client {
        Client {

        }
    }
}