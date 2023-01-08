use actix_web::{error, web, Responder, Result};
use diesel::prelude::*;

use crate::{models::Organization, DbPool};

/// View Org data
/// path -> org id
pub async fn view(path: web::Path<u32>, conn: web::Data<DbPool>) -> Result<impl Responder> {
    use crate::schema::app_organizations::dsl::*;

    // get id that user sendt
    let org_id = path.into_inner();

    let organization = web::block(move || {
        let mut conn = conn.get().unwrap();

        // filter the org by id
        let org_from_id = app_organizations
            .filter(id.eq(org_id as i32))
            .load::<Organization>(&mut conn);

        org_from_id
    })
    .await
    .unwrap();

    match organization {
        Ok(org) => match org.get(0) {
            Some(org) => Ok(web::Json(org.clone())),

            None => Err(error::ErrorNotFound("Not found")),
        },

        Err(_error) => Err(error::ErrorInternalServerError("Some thing is not right!")),
    }
}
