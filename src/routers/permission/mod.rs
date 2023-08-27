use crate::models::PermissionCondition;
use serde::Serialize;
use uuid::Uuid;

pub mod permissions_list;

#[derive(Serialize, Eq, Ord, Hash, Debug, Clone, PartialEq, PartialOrd)]
pub struct SimplePermission {
    uuid: Uuid,
    subject: String,
    object: String,
    action: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PermissionWithConditions {
    #[serde(flatten)]
    permission: SimplePermission,
    conditions: Vec<Option<PermissionCondition>>,
}
