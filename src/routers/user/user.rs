use actix_web::web::{self, ReqData};
use chrono::NaiveDate;
use diesel::prelude::*;
use serde::Serialize;

use crate::error::RouterError;
use crate::models::{Account, Email, User, UserName};
use crate::DbPool;

#[derive(Serialize)]
pub struct FullUserProfile {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub birthday: Option<NaiveDate>,
    pub profile_image: Option<String>,
}

pub async fn view_user(
    pool: web::Data<DbPool>,
    data: ReqData<u32>,
) -> Result<web::Json<FullUserProfile>, RouterError> {
    use crate::schema::app_accounts::dsl::{app_accounts, id as id_from_accounts};
    use crate::schema::app_user_names::dsl::primary_name;

    // Get userId from token Checker
    let acc_id = data.into_inner();

    // select user form db
    // with user_id
    let user: Result<web::Json<FullUserProfile>, RouterError> = web::block(move || {
        let mut conn = pool.get().unwrap();

        let account = app_accounts
            .filter(id_from_accounts.eq(acc_id as i32))
            .load::<Account>(&mut conn)?;

        let Some(account)= account.get(0) else {
            return Err(RouterError::NotFound("Account not found".to_string()));
        };

        let user = User::belonging_to(account).load::<User>(&mut conn)?;

        let Some(user) = user.get(0) else {
            return Err(RouterError::NotFound("User not found".to_string()));
        };

        let email = Email::belonging_to(account).first::<Email>(&mut conn)?;

        // Now get the user names
        let names = UserName::belonging_to(account)
            .filter(primary_name.eq(true))
            .load::<UserName>(&mut conn)?;

        // Is user have any names ?
        let names = if names.is_empty() { None } else { Some(names) };

        let profile = match names {
            Some(names) => {
                // Its must be always 1 element
                let name: &UserName = names.get(0).unwrap();

                FullUserProfile {
                    id: account.id,
                    email: email.email,
                    username: account.username.to_owned(),
                    first_name: Some(name.first_name.to_owned()),
                    last_name: Some(name.last_name.to_owned()),
                    birthday: user.clone().birthday,
                    profile_image: user.clone().profile_image,
                }
            }

            None => FullUserProfile {
                id: account.id,
                email: email.email,
                username: account.username.to_owned(),
                first_name: None,
                last_name: None,
                birthday: user.clone().birthday,
                profile_image: user.clone().profile_image,
            },
        };

        Ok(web::Json(profile))
    })
    .await
    .unwrap();

    user
}
