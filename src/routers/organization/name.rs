use std::str::FromStr;

use actix_web::web;
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
    path: web::Path<String>,
    pool: web::Data<DbPool>,
    new_name_req: web::Json<NewName>,
) -> Result<&'a str, RouterError> {
    use crate::schema::app_accounts::dsl::{app_accounts, uuid as uuid_from_account};
    use crate::schema::app_organization_names::dsl::*;

    let new_name = new_name_req.into_inner();
    let org_uuid = path.into_inner();

    validate(&new_name)?;

    let result: Result<&'a str, RouterError> = web::block(move || {
        let mut conn = pool.get().unwrap();

        let account = app_accounts
            .filter(uuid_from_account.eq(Uuid::from_str(org_uuid.as_str())?))
            .load::<Account>(&mut conn)?;

        let Some(account) = account.get(0) else {
            return Err(RouterError::NotFound("Account not found".to_string()));
        };

        NewOrganizationName {
            account_id: account.id,
            name: new_name.name,
            language: new_name.language,
        }
        .insert_into(app_organization_names)
        .get_result::<OrganizationName>(&mut conn)?;

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
    use crate::schema::app_accounts::dsl::{app_accounts, uuid as uuid_from_account};
    use crate::schema::app_organization_names::dsl::app_organization_names;
    use crate::schema::app_organizations::dsl::app_organizations;

    let path = path.into_inner();

    let result: Result<web::Json<Vec<OrganizationName>>, RouterError> = web::block(move || {
        let mut conn = pool.get().unwrap();

        let id = Uuid::parse_str(&path)?;

        let names = app_organizations
            .inner_join(app_accounts.inner_join(app_organization_names))
            .filter(uuid_from_account.eq(id))
            .select(OrganizationName::as_select())
            .load::<OrganizationName>(&mut conn)?;

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

        let id = Uuid::parse_str(&name_uuid)?;

        diesel::update(app_organization_names.filter(uuid.eq(id)))
            .set((name_name.eq(new_name.name),))
            .execute(&mut conn)?;

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
        let id = Uuid::parse_str(&name_uuid)?;

        diesel::delete(app_organization_names.filter(uuid.eq(id))).execute(&mut conn)?;

        Ok("Deleted")
    })
    .await
    .unwrap();

    result
}
