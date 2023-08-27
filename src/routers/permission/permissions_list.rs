use crate::{
    error::RouterError,
    models::{Permission, PermissionCondition},
    DbPool, routers::permission::PermissionWithConditions,
};
use actix_web::web;
use diesel::prelude::*;

/// Returns the list of Permissions
///
/// with related Conditions
pub async fn get_list_of_permissions(
    pool: web::Data<DbPool>,
) -> Result<web::Json<Vec<Permission>>, RouterError> {
    use crate::schema::app_permission_conditions::dsl::app_permission_conditions;
    use crate::schema::app_permissions::dsl::app_permissions;

    let permissions: Result<PermissionWithConditions, RouterError> = web::block(move || {
        let mut conn = pool.get().unwrap();

        let permissions_with_conditions: Vec<(Permission, PermissionCondition)> = app_permissions
            .inner_join(app_permission_conditions)
            .select((Permission::as_select(), PermissionCondition::as_select()))
            .load(&mut conn)?;

        todo!()
    })
    .await
    .unwrap();

    todo!()
}
