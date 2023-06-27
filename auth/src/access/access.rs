use casbin::{CoreApi, EnforceArgs, Enforcer};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub enum AccessError {
    // When Access Cant Lock
    Lock,

    // No Enforcer With this name
    NotFound,

    // idk
    Enforce,
}

/// First add the enforcers,
/// and then use the finish return
/// and pass it to the routers
pub struct Access {
    /// All the enforcers ( the policy )
    enforcers: Arc<RwLock<HashMap<String, Enforcer>>>,
}

impl Access {
    /// Creates new access module
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            enforcers: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Adds a new enforcer
    pub fn add_enforcer(&self, name: String, enforcer: Enforcer) -> Result<(), AccessError> {
        let Ok(mut locked) = self.enforcers.write() else {
            return Err(AccessError::Lock);
        };

        locked.insert(name, enforcer);

        Ok(())
    }

    /// Remove existing enforcer
    pub fn remove_enforcer(&self, name: &String) -> Result<(), AccessError> {
        let Ok(mut locked) = self.enforcers.write() else {
            return Err(AccessError::Lock);
        };

        locked.remove(name);

        Ok(())
    }

    /// Check the requested Permission, returns bool
    /// If not error
    pub fn enforce<T>(&self, name: &str, args: T) -> Result<bool, AccessError>
    where
        T: EnforceArgs,
    {
        // This will not lock the enforcers
        let Ok(read) = self.enforcers.read() else {
            return Err(AccessError::Lock);
        };

        // Get the required Enforcer
        let Some(enforcer) = read.get(name) else {
            return Err(AccessError::NotFound);
        };

        // Now check
        let Ok(res) = enforcer.enforce(args) else {
            return Err(AccessError::Enforce);
        };

        // false -> Access denied
        // true  -> Access granted
        Ok(res)
    }
}
