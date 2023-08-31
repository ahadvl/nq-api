use crate::{
    authz::{Condition, ConditionValueType, ModelAttrib, ModelAttribResult},
    error::RouterError,
    models::{NewPermission, NewPermissionCondition, Permission},
    DbPool,
};
use actix_web::web;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::SimpleCondition;

#[derive(Serialize, Deserialize)]
pub struct NewPermissionData {
    subject: String,
    object: String,
    action: String,
    conditions: Vec<SimpleCondition>,
}

pub async fn add_permission<'a>(
    new_permission: web::Json<NewPermissionData>,
    pool: web::Data<DbPool>,
) -> Result<&'a str, RouterError> {
    use crate::schema::app_permission_conditions::dsl::app_permission_conditions;
    use crate::schema::app_permissions::dsl::app_permissions;

    let new_permission_data = new_permission.into_inner();

    web::block(move || {
        let mut conn = pool.get().unwrap();

        // First Insert a brand new Permission
        let new_permission: Permission = NewPermission {
            subject: &new_permission_data.subject,
            object: &new_permission_data.object,
            action: &new_permission_data.action,
        }
        .insert_into(app_permissions)
        .get_result(&mut conn)?;

        // Now We must insert the Conditions
        // however We must make sure the request conditions
        // actualy exists
        let mut insertable_conditions: Vec<NewPermissionCondition> = Vec::new();

        for condition in new_permission_data.conditions {
            let model_attr = ModelAttrib::try_from(condition.name.as_str())?;
            let attr_result = ModelAttribResult::from(model_attr);
            let value_type = attr_result.get_value_type();

            let request_value_type = ConditionValueType::try_from(condition.value.as_str())?;

            if value_type != request_value_type {
                return Err(RouterError::BadRequest(
                    "Condition value type is not correct!".to_string(),
                ));
            }

            insertable_conditions.push(NewPermissionCondition {
                permission_id: new_permission.id,
                name: condition.name,
                value: condition.value,
            });
        }

        insertable_conditions
            .insert_into(app_permission_conditions)
            .execute(&mut conn)?;

        Ok(())
    })
    .await
    .unwrap()?;

    Ok("Added")
}
