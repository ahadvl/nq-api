use casbin::{CoreApi, EnforceArgs, Enforcer, Error as CasbinError};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// The kind of AccessError
///
/// Lock -> Error when locking some data
///
/// NotFound -> Something not found
///
/// Enforce -> Error when enforcing
#[derive(Debug, Clone)]
pub enum AccessErrorKind {
    // When Access Cant Lock
    Lock,

    // No Enforcer With this name
    NotFound,

    // idk
    Enforce,
}

#[derive(Debug, Clone)]
pub struct AccessError<'a> {
    /// Kind of error
    pub kind: AccessErrorKind,
    
    /// Give more details like message
    pub detail: &'a str,

    //TODO: Change it to the Option<T>
    // T: Debug
    /// Pass the object for more detail
    pub debug: Option<String>,
}

impl<'a> AccessError<'a> {
    pub fn new(kind: AccessErrorKind, detail: &'a str, debug: Option<String>) -> Self {
        Self {
            kind,
            detail,
            debug,
        }
    }

    /// Used when `something.read()` returns error
    pub fn cant_read(value: String) -> Self {
        Self {
            kind: AccessErrorKind::Lock,
            debug: Some(value),
            detail: "Can't read the value!",
        }
    }
}

impl<'a> From<CasbinError> for AccessError<'a> {
    fn from(_value: CasbinError) -> Self {
        // TODO
        Self {
            kind: AccessErrorKind::Enforce,
            detail: "casbin error",
            debug: None,
        }
    }
}

/// The context for the Access Object
/// holds all the enforcers of app
///
/// The main responsibility of this object is to store the
/// enforcer before server runs.
///
/// its will lock when add_enforcer called
/// but not when read enforcers
pub struct AccessContext {
    /// All the enforcers ( the policy )
    enforcers: HashMap<String, Arc<RwLock<Enforcer>>>,
}

impl AccessContext {
    pub fn new() -> Self {
        Self {
            enforcers: HashMap::new(),
        }
    }

    /// Adds a new enforcer
    ///
    /// - will lock
    pub fn add_enforcer(&mut self, name: String, enforcer: Enforcer) -> Result<(), AccessError> {
        self.enforcers.insert(name, Arc::new(RwLock::new(enforcer)));

        Ok(())
    }

    /// Remove existing enforcer
    ///
    /// - will lock
    pub fn remove_enforcer(&mut self, name: &String) -> Result<(), AccessError> {
        self.enforcers.remove(name);

        Ok(())
    }

    /// Get's the enforcer
    ///
    /// - will not lock
    pub fn get_enforcer(&self, name: &str) -> Result<Arc<RwLock<Enforcer>>, AccessError> {
        // Get the required Enforcer
        let Some(enforcer) = self.enforcers.get(name) else {
            return Err(AccessError::new(AccessErrorKind::NotFound, "Enforcer not found!", None));
        };

        Ok(enforcer.clone())
    }
}

/// Doing the actual work
///
/// will get the enforcer and check the args
pub struct Access {
    context: AccessContext,
}

impl Access {
    pub fn new(context: AccessContext) -> Self {
        Self { context }
    }

    /// Check the requested Permission, returns bool
    /// If not error
    pub fn enforce<T>(&self, name: &str, args: T) -> Result<bool, AccessError>
    where
        T: EnforceArgs,
    {
        let enforcer = self.context.get_enforcer(name)?;

        let Ok(enforcer) = enforcer.read() else {
            return Err(AccessError::cant_read("enforcer".to_string()));
        };

        // Now check
        let res = match enforcer.enforce(args) {
            Ok(res) => Ok(res),

            Err(error) => Err(AccessError::new(
                AccessErrorKind::Enforce,
                "Can't run the enforce function: ",
                Some(format!("{:?}", error)),
            )),
        }?;

        // false -> Access denied
        // true  -> Access granted
        Ok(res)
    }

    /// Adds a new policy
    ///
    /// in other word its kind of,
    /// like giving permission to some user.
    ///
    /// the new policy will added to the enforcer's adapter
    ///
    /// - keep in mind the adapter can be blocking
    /// - This will lock (but its not used that much to begin woried about)
    ///
    /// example usage
    ///
    /// ```rust
    /// access.add_policy("default", vec!["sub", "obj", "action"]);
    /// ```
    pub async fn add_policy(&mut self, enforcer_name: &str, rule: Vec<String>) -> Result<(), AccessError> {
        // get the enforcer
        let enforcer = self.context.get_enforcer(enforcer_name)?;

        let Ok(mut enforcer) = enforcer.write() else {
            //TODO
            return Err(AccessError::cant_read("enforcer".to_string()));
        };

        // get the adapter of the enforcer as mut
        let adapter = enforcer.get_mut_adapter();

        // now add the policy
        //
        // idk what is the meaning of first param
        // so I left it empty :)
        adapter.add_policy("", enforcer_name, rule).await?;

        Ok(())
    }
 }
