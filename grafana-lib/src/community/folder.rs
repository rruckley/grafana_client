//! Folder Module
//! 
use log::info;
use serde::Deserialize;

/// Data model for a folder
#[derive(Debug,Default,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FolderModel {
    /// Numerical Id
    pub id      : u32,
    /// Text-based ID
    pub uid     : Option<String>,
    /// Folder name
    pub title : String,
    /// URL to GUI object
    pub url     : Option<String>,
    /// Type of object
    pub r#type  : Option<String>,
    /// Optional Tag array
    pub tags    : Option<Vec<String>>,
    /// Is this folder starred?
    pub is_starred  : bool,
    /// Legacy URI 
    pub uri     : Option<String>,
}

impl FolderModel {
    /// Create empty FolderModel instance
    pub fn new(title : String) -> FolderModel {
        FolderModel { 
            id: 0,
            uid: None, 
            title,
            url: None,
            r#type: None,
            tags: None,
            is_starred: false,
            uri: None 
        }
    }
}

/// Folder Struct
#[derive(Debug,Default)]
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
        info!("Creating new dashboard: {}",model.title);
        self.model = Some(model);
        self
    }
    /// Send folder to Grafana
    pub fn send(&self) -> Result<String,String> {
        Err(String::from("Create Folder: Not implemented"))
    }
}