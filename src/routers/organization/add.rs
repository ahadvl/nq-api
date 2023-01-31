use actix_web::web;
use diesel::{dsl::exists, prelude::*, select};

use crate::{
    error::RouterError,
    models::{Account, NewAccount, NewOrganization},
    validate::validate,
    DbPool,
};

use super::new_organization_info::NewOrgInfo;

/// Add a new Org
pub async fn add(
    conn: web::Data<DbPool>,
    new_org: web::Json<NewOrgInfo>,
) -> Result<String, RouterError> {
    use crate::schema::app_accounts::dsl::*;
    use crate::schema::app_organizations::dsl::*;

    let new_org_info = new_org.into_inner();

    validate(&new_org_info)?;

    let add_status: Result<String, RouterError> = web::block(move || {
        let mut conn = conn.get().unwrap();

        // Check if org already exists
        let org_exists = select(exists(
            app_accounts.filter(username.eq(&new_org_info.username)),
        ))
        .get_result::<bool>(&mut conn)
        .unwrap();

        if org_exists {
            return Err(RouterError::NotAvailable("username".to_string()));
        }

        // Create new account for org
        let new_account: Account = NewAccount {
            username: &new_org_info.username,
            account_type: &String::from("organization"),
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

        Ok("Created".to_string())
    })
    .await
    .unwrap();

    add_status
}
