use actix_web::{web, Responder, Result};
use diesel::{dsl::exists, prelude::*, select};

use crate::{
    models::{Account, NewAccount, NewOrganization},
    validate::validate,
    DbPool,
};

use super::new_organization_info::NewOrgInfo;

enum OrganizationCreationStatus {
    /// New org created
    Created,

    /// Org with that username exists
    Exists,
}

/// Add a new Org
pub async fn add(
    conn: web::Data<DbPool>,
    new_org: web::Json<NewOrgInfo>,
) -> Result<impl Responder> {
    use crate::schema::app_accounts::dsl::*;
    use crate::schema::app_organizations::dsl::*;

    let new_org_info = new_org.into_inner();

    validate(&new_org_info)?;

    let add_status: OrganizationCreationStatus = web::block(move || {
        let mut conn = conn.get().unwrap();

        // Check if org already exists
        let org_exists = select(exists(
            app_accounts.filter(username.eq(&new_org_info.username)),
        ))
        .get_result::<bool>(&mut conn)
        .unwrap();

        if org_exists {
            return OrganizationCreationStatus::Exists;
        }

        // Create new account for org
        let new_account: Account = NewAccount {
            username: &new_org_info.username,
        }
        .insert_into(app_accounts)
        .get_result(&mut conn)
        .unwrap();

        let _new_organization = NewOrganization {
            account_id: new_account.id,
            name: new_org_info.name,
            profile_image: new_org_info.profile_image,
            established_date: new_org_info.established_date,
            national_id: new_org_info.national_id,
        }
        .insert_into(app_organizations)
        .execute(&mut conn)
        .unwrap();

        OrganizationCreationStatus::Created
    })
    .await
    .unwrap();

    match add_status {
        OrganizationCreationStatus::Created => Ok("created"),
        OrganizationCreationStatus::Exists => Ok("username is not available"),
    }
}
