//! API Module
//! 

use serde::{Serialize,Deserialize};

use log::{info,debug,error};

/// Low level API functions
#[derive(Debug,PartialEq,Default,Serialize,Deserialize)]
pub struct Api {
    token   : String,
    /// Hostname for Grafana
    pub host    : String,
}

impl Api {
    /// Create new Api instance
    pub fn new(host : String,token : String) -> Api {
        Api {
            token,
            host : format!("{}/api",host),
        }
    }
    /// Perform GET operation against Grafana using blocking
    pub fn get(&self, path : String) -> Result<String,String> {
        let url = format!("{}/{}",self.host, path);
        match reqwest::blocking::get(url) {
            Ok(r) => {
                info!("GET responded with status: {}",r.status());
                Ok(r.text().unwrap())
            },
            Err(e) => {
                error!("GET Failed: {}",e.to_string());
                Err(e.to_string())
            }
        }
    }
    /// Perform GET operation against Grafana using async
    pub async fn get_async(&self, path : String) -> Result<String,String> {
        let url = format!("{}/{}",self.host,path);
        debug!("URL: {url}");
        let _body = reqwest::get(url).await.unwrap();
        Ok(String::from("It was good"))
    }

    /// Send compatible struct through to Grafana using async
    pub async fn post_async<T>(self, payload : T) -> Result<String,String> 
    where T : Sized + Serialize,
    {
        let client = reqwest::Client::new();
        match client.post(self.host)
            .json(&payload)
            .bearer_auth(self.token)
            .send().await {
                Ok(_) => Ok("Yay".to_string()),
                Err(e) => Err(e.to_string()),
            }
    }
}