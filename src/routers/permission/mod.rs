use crate::models::PermissionCondition;

pub mod permissions_list;


#[derive(Debug, Clone)]
pub struct PermissionWithConditions {
    subject: String,
    object: String,
    action: String,
    conditions: Vec<PermissionCondition>,
}

