use actix_web::web::{self, ReqData};
use chrono::NaiveDate;
use diesel::prelude::*;
use serde::Serialize;

use crate::error::RouterError;
use crate::models::{Account, Email, User};
use crate::DbPool;

#[derive(Serialize)]
pub struct FullUserProfile {
    pub email: String,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub birthday: Option<NaiveDate>,
    pub profile_image: Option<String>,
}

pub async fn view_profile(
    pool: web::Data<DbPool>,
    data: ReqData<u32>,
) -> Result<web::Json<FullUserProfile>, RouterError> {
    use crate::schema::app_accounts::dsl::{app_accounts, id as id_from_accounts};

    // Get userId from token Checker
    let acc_id = data.into_inner();

    // select user form db
    // with user_id
    let user_profile: Result<FullUserProfile, RouterError> = web::block(move || {
        let mut conn = pool.get().unwrap();

        let account = app_accounts
            .filter(id_from_accounts.eq(acc_id as i32))
            .load::<Account>(&mut conn)
            .unwrap();

        let Some(account)= account.get(0) else {
            return Err(RouterError::NotFound);
        };

        let user = User::belonging_to(account).load::<User>(&mut conn).unwrap();

        let user = user.get(0).unwrap();

        let email = Email::belonging_to(account)
            .load::<Email>(&mut conn)
            .unwrap();

        let email: &Email = email.get(0).unwrap();

        let profile = FullUserProfile {
            email: email.clone().email,
            username: account.username.to_owned(),
            first_name: user.clone().first_name,
            last_name: user.clone().last_name,
            birthday: user.clone().birthday,
            profile_image: user.clone().profile_image,
        };

        Ok(profile)
    })
    .await
    .unwrap();

    if let Ok(profile) = user_profile {
        Ok(web::Json(profile))
    } else {
        Err(user_profile.err().unwrap())
    }
}
