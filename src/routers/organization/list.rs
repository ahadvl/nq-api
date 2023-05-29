use std::collections::BTreeMap;

use crate::{
    datetime::parse_date_time_with_format,
    error::RouterError,
    models::{Account, Organization, OrganizationName},
    routers::{multip, organization::new_organization_info::NewOrgInfo},
    DbPool,
};
use actix_web::web;
use chrono::NaiveDate;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize)]
pub struct OrgWithName {
    pub primary_name: String,
    pub profile_image: Option<String>,
    pub established_date: NaiveDate,
    pub national_id: String,
}

#[derive(Serialize)]
pub struct OrganizationWithUsername {
    id: i32,
    username: String,
    org: SimpleOrg,
}

#[derive(Serialize)]
pub struct OrgWithNames {
    #[serde(flatten)]
    org: SimpleOrg,
    names: Vec<OrganizationName>,
}

#[derive(Deserialize, Validate, Serialize, PartialOrd, Ord, Eq, Hash, PartialEq)]
struct SimpleOrg {
    pub username: String,
    pub profile_image: Option<String>,

    #[serde(deserialize_with = "parse_date_time_with_format")]
    pub established_date: NaiveDate,

    #[validate(length(equal = 11))]
    pub national_id: String,
}

pub async fn get_list_of_organizations(
    pool: web::Data<DbPool>,
) -> Result<web::Json<Vec<OrganizationWithUsername>>, RouterError> {
    use crate::schema::app_accounts::dsl::app_accounts;
    use crate::schema::app_organization_names::dsl::app_organization_names;
    use crate::schema::app_organizations::dsl::app_organizations;

    let organizations: Result<Vec<OrganizationWithUsername>, RouterError> = web::block(move || {
        let mut conn = pool.get().unwrap();

        let Ok(all_orgs_with_names) = app_organizations
            .inner_join(app_accounts.inner_join(app_organization_names))
            .select((Organization::as_select(),  OrganizationName::as_select()))
            .load::<(Organization, OrganizationName)>(&mut conn)
            else {
                return Err(RouterError::InternalError);
            };

        let orgs_with_names: BTreeMap<SimpleOrg, Vec<OrganizationName>> =
            multip(all_orgs_with_names, |org| SimpleOrg {
                username: org.username,
                profile_image: org.profile_image,
                established_date: org.established_date,
                national_id: org.national_id,
            });

        let result = select_all
            .into_iter()
            .map(|(org, account, name)| OrganizationWithUsername {
                id: org.id,
                username: account.username,
                org: OrgWithName {
                    primary_name: name.name,
                    profile_image: org.profile_image,
                    established_date: org.established_date,
                    national_id: org.national_id,
                },
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
