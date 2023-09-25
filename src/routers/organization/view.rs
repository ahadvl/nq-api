use std::str::FromStr;

use actix_web::web;
use chrono::NaiveDate;
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

use crate::error::RouterError;
use crate::models::OrganizationName;
use crate::{
    models::{Account, Organization},
    DbPool,
};

#[derive(Serialize, Clone)]
pub struct ViewableOrganizationData {
    pub username: String,
    pub name: String,
    pub profile_image: Option<String>,
    pub established_date: NaiveDate,
    pub national_id: String,
}

/// View Org data
/// path -> account_uuid related to the org
pub async fn view(
    path: web::Path<String>,
    conn: web::Data<DbPool>,
) -> Result<web::Json<ViewableOrganizationData>, RouterError> {
    use crate::schema::app_accounts::dsl::{app_accounts, uuid as acc_uuid};
    use crate::schema::app_organization_names::dsl::language;

    let account_uuid = path.into_inner();

    let organization: Result<web::Json<ViewableOrganizationData>, RouterError> =
        web::block(move || {
            let mut conn = conn.get().unwrap();

            let uuid = Uuid::from_str(&account_uuid)?;

            // Find the account
            let account = app_accounts
                .filter(acc_uuid.eq(uuid))
                .load::<Account>(&mut conn)?;

            let Some(account) = account.get(0) else {
                return Err(RouterError::NotFound("Account not found".to_string()));
            };

            let org = Organization::belonging_to(account).load::<Organization>(&mut conn)?;

            let Some(org) = org.get(0) else {
                return Err(RouterError::NotFound("Organization not found".to_string()));
            };

            let org_name = OrganizationName::belonging_to(account)
                .filter(language.eq("default"))
                .first::<OrganizationName>(&mut conn)?;

            let org = ViewableOrganizationData {
                username: account.username.clone(),
                name: org_name.name.clone(),
                profile_image: org.profile_image.clone(),
                established_date: org.established_date,
                national_id: org.national_id.clone(),
            };

            Ok(web::Json(org))
        })
        .await
        .unwrap();

    organization
}
