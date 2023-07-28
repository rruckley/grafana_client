//! Authentication Module
//! 


/// Authentication Keys
pub struct AuthenticationKey {
    /// Key Id
    pub id          : u16,
    /// Key Name
    pub name        : String,
    /// Key Role
    pub role        : String,
    /// Key Expiratoin (Optional)
    pub expiration  : Option<String>,
}

/// Authentication Structure
pub struct Authentication {}

impl Authentication {
    /// Return list of configured API Keys
    pub fn keys() -> Result<Vec<AuthenticationKey>,String> {
        Err(String::from("Notimplemented"))
    }
    /// Create a new Authentication key
    pub fn create(_key : AuthenticationKey) -> Result<AuthenticationKey,String> {
        Err(String::from("Not Implemented"))
    }

    /// Delete an authentication key
    pub fn delete(_key : AuthenticationKey) -> Result<(),String> {
        Err(String::from("Not implemented"))
    }
}