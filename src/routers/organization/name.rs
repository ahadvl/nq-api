use actix_web::web::{self, ReqData};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    error::RouterError,
    models::{Account, NewOrganizationName, OrganizationName},
    validate::validate,
    DbPool,
};
use validator::Validate;

#[derive(Validate, Deserialize, Serialize)]
pub struct NewName {
    name: String,

    #[validate(length(equal = 2))]
    language: String,
}

pub async fn add_name(
    pool: web::Data<DbPool>,
    new_name_req: web::Json<NewName>,
    data: ReqData<u32>,
) -> Result<String, RouterError> {
    use crate::schema::app_accounts::dsl::{app_accounts, id as acc_id};
    use crate::schema::app_organization_names::dsl::*;

    let new_name = new_name_req.into_inner();
    let user_account_id = data.into_inner();

    validate(&new_name)?;

    let result: Result<String, RouterError> = web::block(move || {
        let mut conn = pool.get().unwrap();

        let Ok(account) = app_accounts
            .filter(acc_id.eq(user_account_id as i32))
            .load::<Account>(&mut conn) else {
                return Err(RouterError::InternalError);
            };

        let Some(account) = account.get(0) else {
            return Err(RouterError::NotFound("Account not found".to_string()));
        };

        let Ok(_new_name) = NewOrganizationName {
                account_id: account.id,
                name: new_name.name,
                language: new_name.language,
            }
            .insert_into(app_organization_names)
            .get_result::<OrganizationName>(&mut conn) else {
                return Err(RouterError::InternalError);
            };

        Ok("Added".to_string())
    })
    .await
    .unwrap();

    result
}
