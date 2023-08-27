use crate::{
    error::RouterError,
    models::{Permission, PermissionCondition},
    DbPool,
};
use actix_web::web;
use diesel::prelude::*;

#[derive(Debug, Clone)]
struct PermissionWithConditions {
    subject: String,
    object: String,
    action: String,
    conditions: Vec<PermissionCondition>,
}

/// Returns the list of Permissions
///
/// with related Conditions
pub async fn get_list_of_permissions(
    pool: web::Data<DbPool>,
) -> Result<web::Json<Vec<Permission>>, RouterError> {
    use crate::schema::app_permission_conditions::dsl::app_permission_conditions;
    use crate::schema::app_permissions::dsl::app_permissions;

    let permissions = web::block(move || {
        let mut conn = pool.get().unwrap();

        let permissions_with_conditions: Vec<()> = app_permissions
            .inner_join(app_permission_conditions)
            .select((Permission::as_select(), PermissionCondition::as_select()))
            .load(&mut conn);
    })
    .await
    .unwrap();

    todo!()
}
