//! Error Module
//! 
//! 
use std::error::Error;

/// Error Structure
#[derive(Debug)]
pub struct GrafanaError {
    /// Error Message
    pub message : String,
    /// Error Status Code
    pub status : String,
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