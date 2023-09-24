use std::str::FromStr;

use crate::error::RouterError;
use crate::DbPool;
use ::uuid::Uuid;
use actix_web::web;
use diesel::prelude::*;

/// Delete's a single organization
pub async fn delete_organization<'a>(
    path: web::Path<String>,
    pool: web::Data<DbPool>,
) -> Result<&'a str, RouterError> {
    use crate::schema::app_organizations::dsl::{app_organizations, uuid as org_uuid};

    let path = path.into_inner();

    let result = web::block(move || {
        let mut conn = pool.get().unwrap();
        let uuid = Uuid::from_str(&path)?;

        // remove uuid
        diesel::delete(app_organizations.filter(org_uuid.eq(uuid))).execute(&mut conn)?;

        Ok("Deleted")
    })
    .await
    .unwrap();

    result
}
