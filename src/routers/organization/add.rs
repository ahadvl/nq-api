use actix_web::web::{self, ReqData};
use auth::access::access::Access;
use diesel::{dsl::exists, prelude::*, select};
use std::sync::Arc;

use crate::{
    error::RouterError,
    models::{
        Account, NewAccount, NewEmployee, NewOrganization, NewOrganizationName, Organization,
    },
    validate::validate,
    DbPool,
};

use super::new_organization_info::NewOrgInfo;

/// Add a new Org
pub async fn add<'a>(
    conn: web::Data<DbPool>,
    new_org: web::Json<NewOrgInfo>,
    data: ReqData<u32>,
    access: web::Data<Arc<Access>>,
) -> Result<&'a str, RouterError> {
    use crate::schema::app_accounts::dsl::*;
    use crate::schema::app_employees::dsl::app_employees;
    use crate::schema::app_organization_names::dsl::app_organization_names;
    use crate::schema::app_organizations::dsl::app_organizations;

    let new_org_info = new_org.into_inner();
    let user_account_id = data.into_inner();
    let access = access.into_inner();

    validate(&new_org_info)?;

    let add_status: Result<u32, RouterError> = web::block(move || {
        let mut conn = conn.get().unwrap();

        // Check if org already exists
        let org_exists = select(exists(
            app_accounts.filter(username.eq(&new_org_info.username)),
        ))
        .get_result::<bool>(&mut conn)?;

        if org_exists {
            return Err(RouterError::NotAvailable(
                "organization username".to_string(),
            ));
        }

        // Create new account for org
        let new_account = NewAccount {
            username: &new_org_info.username,
            account_type: &String::from("organization"),
        }
        .insert_into(app_accounts)
        .get_result::<Account>(&mut conn)?;

        let new_organization = NewOrganization {
            account_id: new_account.id,
            profile_image: new_org_info.profile_image,
            established_date: new_org_info.established_date,
            national_id: new_org_info.national_id,
        }
        .insert_into(app_organizations)
        .get_result::<Organization>(&mut conn)?;

        // Now add the creator user as employee to the organization
        let user_account = app_accounts
            .filter(id.eq(user_account_id as i32))
            .get_result::<Account>(&mut conn)?;

        NewEmployee {
            employee_account_id: user_account.id,
            org_account_id: new_organization.account_id,
        }
        .insert_into(app_employees)
        .execute(&mut conn)?;

        // Add new name to the org
        NewOrganizationName {
            account_id: new_account.id,
            language: "default".to_string(),
            name: new_org_info.name,
        }
        .insert_into(app_organization_names)
        .execute(&mut conn)?;

        Ok(new_account.id as u32)
    })
    .await
    .unwrap();

    let add_status = add_status?;

    // TODO: dont use unwrap impl From trait for errors
    access
        .add_policy(
            "access",
            "p",
            vec![
                user_account_id.to_string(),
                format!("org:{}", add_status),
                "write".to_string(),
            ],
        )
        .await?;

    Ok("Created")
}
