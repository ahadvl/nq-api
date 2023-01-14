use actix_web::error::{self};
use actix_web::http::StatusCode;
use actix_web::{web, Responder, Result};
use chrono::NaiveDate;
use diesel::prelude::*;
use serde::Serialize;

use crate::{
    models::{Account, Organization},
    DbPool,
};

#[derive(Serialize, Clone)]
struct ViewableOrganizationData {
    pub id: i32,
    pub username: String,
    pub name: String,
    pub profile_image: Option<String>,
    pub established_date: NaiveDate,
    pub national_id: String,
}

/// View Org data
/// path -> org id
pub async fn view(path: web::Path<u32>, conn: web::Data<DbPool>) -> Result<impl Responder> {
    use crate::schema::app_accounts::dsl::*;

    // get id that user sendt
    let org_id = path.into_inner();

    let organization: Result<ViewableOrganizationData, (StatusCode, String)> =
        web::block(move || {
            let mut conn = conn.get().unwrap();

            // Find the account
            let account = app_accounts
                .filter(id.eq(org_id as i32))
                .load::<Account>(&mut conn)
                .unwrap();

            if account.is_empty() {
                return Err((
                    StatusCode::NOT_FOUND,
                    "Account with this id is not exists".to_string(),
                ));
            }

            let account = account.get(0).unwrap();

            let org = Organization::belonging_to(account.clone())
                .load::<Organization>(&mut conn)
                .unwrap();

            let org = org.get(0).unwrap();

            let account_copy = account.clone();

            let org = ViewableOrganizationData {
                id: org.id,
                username: account_copy.username.clone(),
                name: org.name.clone(),
                profile_image: org.profile_image.clone(),
                established_date: org.established_date.clone(),
                national_id: org.national_id.clone(),
            };

            Ok(org)
        })
        .await
        .unwrap();

    match organization {
        Ok(org) => Ok(web::Json(org.clone())),

        Err(_error) => Err(error::ErrorInternalServerError("Something is not right!")),
    }
}
