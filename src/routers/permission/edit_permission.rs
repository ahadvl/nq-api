use std::str::FromStr;

use crate::{
    error::RouterError,
    models::{Permission, NewPermissionCondition},
    DbPool,
};
use actix_web::web::{self, Path};
use diesel::prelude::*;
use uuid::Uuid;

use super::NewPermissionData;

/// Edit's the target permission
///
/// On updating the conditions this router will
/// remove all related conditions, and insert the new ones
/// this solution is kind of stupid but it's really simple.
pub async fn edit_permission<'a>(
    target_permission: Path<String>,
    new_permission: web::Json<NewPermissionData>,
    pool: web::Data<DbPool>,
) -> Result<&'a str, RouterError> {
    use crate::schema::app_permission_conditions::dsl::{app_permission_conditions, permission_id};
    use crate::schema::app_permissions::dsl::{
        action, app_permissions, object, subject, uuid as uuid_of_permission,
    };

    let target_permission = target_permission.into_inner();
    let new_permission = new_permission.into_inner();

    let result: Result<&'a str, RouterError> = web::block(move || {
        let mut conn = pool.get().unwrap();
        let permission_uuid = Uuid::from_str(&target_permission)?;

        let permission: Permission = diesel::update(app_permissions)
            .filter(uuid_of_permission.eq(permission_uuid))
            .set((
                subject.eq(new_permission.subject),
                object.eq(new_permission.object),
                action.eq(new_permission.action),
            ))
            .get_result(&mut conn)?;

        // Delete the related conditions to permission
        diesel::delete(app_permission_conditions.filter(permission_id.eq(permission.id)))
            .execute(&mut conn)?;

        // Now We must insert the Conditions
        // however We must make sure the request conditions
        // actualy exists
        let mut insertable_conditions: Vec<NewPermissionCondition> = Vec::new();

        for condition in new_permission.conditions {
            let _ = condition.validate()?;

            insertable_conditions.push(NewPermissionCondition {
                permission_id: permission.id,
                name: condition.name,
                value: condition.value,
            });
        }

        insertable_conditions
            .insert_into(app_permission_conditions)
            .execute(&mut conn)?;

        Ok("Updated")
    })
    .await
    .unwrap();

    result
}
