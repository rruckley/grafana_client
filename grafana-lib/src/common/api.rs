//! API Module
//! 

use serde::Serialize;

/// Low level API functions
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
    /// Perform GET operation against Grafana
    pub async fn get(&self, url : String) -> Result<String,String> {
        let _body = reqwest::get(url).await.unwrap();
        Ok(String::from("It was good"))
    }

    /// Send compatible struct through to Grafana
    pub async fn post<T>(self, payload : T) -> Result<String,String> 
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