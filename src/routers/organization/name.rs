use actix_web::web::{self, ReqData};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    error::RouterError,
    models::{Account, NewOrganizationName, OrganizationName},
    validate::validate,
    DbPool,
};
use uuid::Uuid;
use validator::Validate;

#[derive(Validate, Deserialize, Serialize)]
pub struct NewName {
    name: String,

    #[validate(length(equal = 2))]
    language: String,
}

pub async fn add_name<'a>(
    pool: web::Data<DbPool>,
    new_name_req: web::Json<NewName>,
    data: ReqData<u32>,
) -> Result<&'a str, RouterError> {
    use crate::schema::app_accounts::dsl::{app_accounts, id as acc_id};
    use crate::schema::app_organization_names::dsl::*;

    let new_name = new_name_req.into_inner();
    let user_account_id = data.into_inner();

    validate(&new_name)?;

    let result: Result<&'a str, RouterError> = web::block(move || {
        let mut conn = pool.get().unwrap();

        let Ok(account) = app_accounts
            .filter(acc_id.eq(user_account_id as i32))
            .load::<Account>(&mut conn) else {
                return Err(RouterError::InternalError);
            };

        let Some(account) = account.get(0) else {
            return Err(RouterError::NotFound("Account not found".to_string()));
        };

        let Ok(_new_name) = NewOrganizationName {
                account_id: account.id,
                name: new_name.name,
                language: new_name.language,
            }
            .insert_into(app_organization_names)
            .get_result::<OrganizationName>(&mut conn) else {
                return Err(RouterError::InternalError);
            };

        Ok("Added")
    })
    .await
    .unwrap();

    result
}

/// Returns the list of org names
pub async fn names(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> Result<web::Json<Vec<OrganizationName>>, RouterError> {
    use crate::schema::app_accounts::dsl::app_accounts;
    use crate::schema::app_organization_names::dsl::app_organization_names;
    use crate::schema::app_organizations::dsl::{app_organizations, uuid};

    let path = path.into_inner();

    let result: Result<web::Json<Vec<OrganizationName>>, RouterError> = web::block(move || {
        let mut conn = pool.get().unwrap();

        let Ok(id) = Uuid::parse_str(&path) else {
            return Err(RouterError::InternalError)
        };

        let Ok(names) = app_organizations
            .inner_join(app_accounts.inner_join(app_organization_names))
            .filter(uuid.eq(id))
            .select(OrganizationName::as_select())
            .load::<OrganizationName>(&mut conn) else {
                return Err(RouterError::InternalError)
            };

        Ok(web::Json(names))
    })
    .await
    .unwrap();

    result
}

#[derive(Deserialize)]
pub struct EditableName {
    /// New name
    name: String,
    // We dont grant user to update the existing
    // names language. the language property of the name is
    // immutable
}

/// Edits the name
pub async fn edit_name<'a>(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
    edit_name_req: web::Json<EditableName>,
) -> Result<&'a str, RouterError> {
    use crate::schema::app_organization_names::dsl::{
        app_organization_names, name as name_name, uuid,
    };

    let name_uuid = path.into_inner();
    let new_name = edit_name_req.into_inner();

    let result = web::block(move || {
        let mut conn = pool.get().unwrap();

        let Ok(id) = Uuid::parse_str(&name_uuid) else {
            return Err(RouterError::BadRequest("Cant parse the uuid!".to_string()));
        };

        let Ok(_) = diesel::update(app_organization_names.filter(uuid.eq(id)))
            .set((
                name_name.eq(new_name.name),
            ))
            .execute(&mut conn)
            else {
                return Err(RouterError::InternalError);
            };

        Ok("Edited")
    })
    .await
    .unwrap();

    result
}

/// Deletes the name as given uuid
pub async fn delete_name<'a>(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> Result<&'a str, RouterError> {
    use crate::schema::app_organization_names::dsl::{app_organization_names, uuid};

    let name_uuid = path.into_inner();

    let result = web::block(move || {
        let mut conn = pool.get().unwrap();

        // Parse the uuid if we can
        let Ok(id) = Uuid::parse_str(&name_uuid) else {
            return Err(RouterError::BadRequest("Cant parse the uuid!".to_string()));
        };

        let Ok(_deleted) =
            diesel::delete(app_organization_names.filter(uuid.eq(id))).execute(&mut conn) else {
                return Err(RouterError::InternalError);
            };

        Ok("Deleted")
    })
    .await
    .unwrap();

    result
}
