//! Folder Module
//! 
use log::info;

/// Data model for a folder
pub struct FolderModel {
    /// Folder name
    pub name : String,
}

/// Folder Struct
pub struct Folder {
    model : Option<FolderModel>,
}

impl Folder {
    /// Create empty instance of folder API
    pub fn new() -> Folder {
        Folder {
            model : None,
        }
    }
    /// Create new folder 
    pub fn create(mut self, model : FolderModel) -> Folder {
        info!("Creating new dashboard: {}",model.name);
        self.model = Some(model);
        self
    }
    /// Send folder to Grafana
    pub fn send(&self) -> Result<String,String> {
        Err(String::from("Create Folder: Not implemented"))
    }
}