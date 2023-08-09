//! Annotations Module
//! 

use crate::common::{api::Api, error::GrafanaError};

use serde::Deserialize;
use std::fmt;

const ANNOTATION_PATH : &str = "annotations";

/// Annotations Model
#[derive(Debug,Default,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnnotationsModel {
    dashboard_id : u32,
    panel_id : u32,
    text : String,
}

impl fmt::Display for AnnotationsModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::default();
        output.push_str(format!("Text\t: {}\n",self.text.clone()).as_str());
        output.push_str(format!("Dashboard\t: {}\n",self.dashboard_id).as_str());
        output.push_str(format!("Panel\t: {}\n",self.panel_id).as_str());
        write!(f, "{output}" )
    }
}

/// Annotations Structure

pub struct Annotations {
    api : Api,
    models : Option<Vec<AnnotationsModel>>,
}

impl Annotations {
    /// Create new instance of Annotations API
    pub fn new(api : Api) -> Annotations {
        Annotations {
            api,
            models : None,
        }
    }

    /// Add model to annotations API for creation
    pub fn with_model(mut self, model : AnnotationsModel) -> Annotations {
        self.models = Some(vec![model]);
        self
    }

    /// Add annotation vector
    pub fn with_model_vec(mut self, model_vec : Vec<AnnotationsModel>) -> Annotations {
        self.models = Some(model_vec);
        self
    }

    /// Get a list of annotations
    /// # Example
    /// ```
    /// # use grafana_lib::client::Client;
    /// # let client = Client::new(String::from("http://localhost:3000"));
    /// let result = client.annotations().list(Some(10));
    /// ```
    pub fn list(&self,limit : Option<u16>) -> Result<Vec<AnnotationsModel>,GrafanaError> {
        let path = match limit {
            Some(l) => format!("{}?limit={}",ANNOTATION_PATH,l),
            None => format!("{}",ANNOTATION_PATH)
        };
        match self.api.get(path) {
            Ok(r) => {
                let result : Vec<AnnotationsModel> = serde_json::from_str(r.as_str()).unwrap();
                Ok(result)
            },
            Err(e) => Err(GrafanaError::new(e,String::from("ERROR")))
        }
    }
}