//! API Module
//! 


/// Low level API functions
struct Api {}

impl Api {
    async fn get(&self, url : String) -> Result<String> {
        let _body = reqwest::get(url).await?;
        Ok(String::from("It was good"))
    }

    /// Send compatible struct through to Grafana
    async fn post<T>(&self, payload : T) -> Result<String> 
    where T : Sized + Serialize,
    {
        let client = reqwest::Client::new();
        match client.post(self.config.url())
            .json(&payload)
            .send().await {
                Ok(_) => Ok("Yay".to_string()),
                Err(e) => Err(e),
            }
    }
}