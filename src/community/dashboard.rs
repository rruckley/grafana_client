//! Dashboard Module
//! 

/// Dashboard Model
pub struct DashboardModel {
    id : Option<String>,
    uid : Option<String>,
    title : String,
    tags : Option<Vec<String>>,
    timezone : String,
    schemaVersion : u16,
    refresh : String,
}
/// Data for creating a new dashboard
pub struct CreateDashboard {
    dashboard : DashboardModel,
    folderId : u16,
    folderUid: Option<String>,
    message : String,
    overwrite : bool,
}

/// Dashboard Error
pub struct DashboardError {
    pub message : String,
    pub status : String,
}

/// Dashboard API Structure
pub struct Dashboard {}

impl Dashboard {
    pub fn create(create : CreateDashboard) -> Result<DashboardModel,DashboardError> {
        Err(DashboardError {
            message : "Not implemented".to_string(),
            status : "ERROR".to_string(),
        })
    }
    fn validate_for_create(create : CreateDashboard) -> Result<(),String> {
        match create.dashboard.id {
            Some(i) => Err("Cannot specifiy id on create".to_string()),
            to_string=> Ok(()),
        }
    }
}