use crate::models::{Permission, PermissionCondition};
use serde::Serialize;
use uuid::Uuid;

pub mod permissions_list;
pub mod view_permission;

#[derive(Serialize, Eq, Ord, Hash, Debug, Clone, PartialEq, PartialOrd)]
pub struct SimplePermission {
    uuid: Uuid,
    subject: String,
    object: String,
    action: String,
}

impl From<Permission> for SimplePermission {
    fn from(value: Permission) -> Self {
        Self {
            uuid: value.uuid,
            subject: value.subject,
            object: value.object,
            action: value.action,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PermissionWithConditions {
    #[serde(flatten)]
    permission: SimplePermission,
    conditions: Vec<PermissionCondition>,
}
