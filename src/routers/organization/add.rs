use actix_web::{web, Responder, Result};
use diesel::{dsl::exists, prelude::*, select};

use crate::{models::NewOrganization, validate::validate, DbPool};

enum OrganizationCreationStatus {
    /// New org created
    Created,

    /// Org with that username exists
    Exists,
}

/// Add a new Org
pub async fn add(
    conn: web::Data<DbPool>,
    new_org: web::Json<NewOrganization>,
) -> Result<impl Responder> {
    use crate::schema::app_organizations_table::dsl::*;

    let new_org = new_org.into_inner();

    validate(&new_org)?;

    let add_status: OrganizationCreationStatus = web::block(move || {
        let mut conn = conn.get().unwrap();

        // Check if org already exists
        let org_exists = select(exists(
            app_organizations_table.filter(username.eq(&new_org.username)),
        ))
        .get_result::<bool>(&mut conn)
        .unwrap();

        if org_exists {
            return OrganizationCreationStatus::Exists;
        }

        // Add a new org to the db
        let _org = diesel::insert_into(app_organizations_table)
            .values(new_org)
            .execute(&mut conn);

        OrganizationCreationStatus::Created
    })
    .await
    .unwrap();

    match add_status {
        OrganizationCreationStatus::Created => Ok("created"),
        OrganizationCreationStatus::Exists => Ok("organization with this username exists"),
    }
}
