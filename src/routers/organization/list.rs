use crate::{
    error::RouterError,
    models::{Account, Organization},
    DbPool,
};
use actix_web::web;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Serialize)]
pub struct OrganizationWithUsername {
    id: i32,
    username: String,
    org: Organization,
}

pub async fn get_list_of_organizations(
    pool: web::Data<DbPool>,
) -> Result<web::Json<Vec<OrganizationWithUsername>>, RouterError> {
    use crate::schema::app_accounts;
    use crate::schema::app_organizations;

    let organizations: Result<Vec<OrganizationWithUsername>, RouterError> = web::block(move || {
        let mut conn = pool.get().unwrap();

        let Ok(select_all) = app_organizations::dsl::app_organizations
            .inner_join(app_accounts::dsl::app_accounts)
            .load::<(Organization, Account)>(&mut conn)
            else {
                return Err(RouterError::InternalError);
            };

        let result = select_all
            .into_iter()
            .map(|(org, account)| OrganizationWithUsername {
                id: org.id,
                username: account.username,
                org,
            })
            .collect::<Vec<OrganizationWithUsername>>();

        Ok(result)
    })
    .await
    .unwrap();

    match organizations {
        Ok(orgs) => Ok(web::Json(orgs)),
        Err(err) => Err(err),
    }
}
