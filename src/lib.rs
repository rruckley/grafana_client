
//! Client library for Grafana
//! 
//! Provides a simple library to use for interacting with Grafana 10+
//! 
//! 

#![warn(missing_docs)]

/// Common modules
pub mod config;
pub mod client;

/// Community modules, included by default
pub mod community;

/// Enterprise modules
#[cfg(feature = "enterprise")]
pub mod enterprise;