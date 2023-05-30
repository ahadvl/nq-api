use actix_web::web;
use chrono::NaiveDate;
use diesel::prelude::*;
use serde::Serialize;

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
/// path -> org id
pub async fn view(
    path: web::Path<u32>,
    conn: web::Data<DbPool>,
) -> Result<web::Json<ViewableOrganizationData>, RouterError> {
    use crate::schema::app_accounts::dsl::*;
    use crate::schema::app_organization_names::dsl::language;

    // get id that user sendt
    let org_id = path.into_inner();

    let organization: Result<ViewableOrganizationData, RouterError> = web::block(move || {
        let mut conn = conn.get().unwrap();

        // Find the account
        let Ok(account)= app_accounts
            .filter(id.eq(org_id as i32))
            .load::<Account>(&mut conn) else {
                return Err(RouterError::InternalError);
            };

        let Some(account) = account.get(0) else {
            return Err(RouterError::NotFound("Account not found".to_string()));
        };

        let Ok(org) = Organization::belonging_to(account)
            .load::<Organization>(&mut conn) else {
                return Err(RouterError::InternalError);
            };

        let Some(org) = org.get(0) else {
            return Err(RouterError::NotFound("Organization not found".to_string()));
        };

        let Ok(org_name) = OrganizationName::belonging_to(account)
            .filter(language.eq("default"))
            .load::<OrganizationName>(&mut conn) else {
                return Err(RouterError::NotFound("Organization not found".to_string()));
            };

        let Some(org_name) = org_name.get(0) else {
            return Err(RouterError::InternalError);
        };

        let org = ViewableOrganizationData {
            username: account.username.clone(),
            name: org_name.name.clone(),
            profile_image: org.profile_image.clone(),
            established_date: org.established_date,
            national_id: org.national_id.clone(),
        };

        Ok(org)
    })
    .await
    .unwrap();

    if let Ok(org) = organization {
        Ok(web::Json(org))
    } else {
        Err(organization.err().unwrap())
    }
}
