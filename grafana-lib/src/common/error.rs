//! Error Module
//! 
//! 
use std::error::Error;

/// Error Structure
#[derive(Debug)]
pub struct GrafanaError {
    /// Error Message
    message : String,
    /// Error Status Code
    status : String,
}

impl GrafanaError {
    /// Create a new instance of GrafanaError passing in a message and status code.
    /// # Example
    /// ```
    /// # use grafana_lib::common::error::GrafanaError;
    /// let err = GrafanaError::new(String::from("A Message"), String::from("A Status"));
    /// ```
    pub fn new(message : String, status : String) -> GrafanaError {
        GrafanaError { message, status }
    }
}

impl std::fmt::Display for GrafanaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} : {}", self.status, self.message)
    }
}

impl std::convert::From<String> for GrafanaError {
    fn from(msg: String) -> Self {
        GrafanaError { message:  msg, status: String::from("simple-error") }
    }
}

impl Error for GrafanaError {

}