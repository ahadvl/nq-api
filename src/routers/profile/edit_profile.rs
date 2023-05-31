use actix_web::web::{self, ReqData};
use chrono::NaiveDate;
use diesel::prelude::*;
use serde::Deserialize;

use crate::{
    error::RouterError,
    models::{Account, User, UserName},
    DbPool,
};

#[derive(Deserialize)]
pub struct EditableProfile {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub birthday: NaiveDate,
    pub profile_image: String,
    pub language: String,
}

/// Edit the profile
/// wants a new profile and token
pub async fn edit_profile<'a>(
    data: ReqData<u32>,
    pool: web::Data<DbPool>,
    new_profile: web::Json<EditableProfile>,
) -> Result<&'a str, RouterError> {
    use crate::schema::app_accounts::dsl::{app_accounts, id as acc_id, username};
    use crate::schema::app_user_names::dsl::{first_name, last_name, primary_name};
    use crate::schema::app_users::dsl::*;

    let user_id = data.into_inner();
    let new_profile = new_profile.into_inner();

    let edit_status: Result<&'a str, RouterError> = web::block(move || {
        let mut conn = pool.get().unwrap();

        // First find the org from id
        let account = app_accounts
            .filter(acc_id.eq(user_id as i32))
            .load::<Account>(&mut conn)?;

        let Some(account) = account.get(0) else {
            return Err(RouterError::NotFound("Account not found".to_string()));
        };

        let user = User::belonging_to(account).load::<User>(&mut conn)?;

        let Some(current_user_profile) = user.get(0) else {
            return Err(RouterError::NotFound("user not found".to_string()));
        };

        // Now update the account username
        diesel::update(account)
            .set(username.eq(new_profile.username))
            .execute(&mut conn)?;

        // And update the other data
        diesel::update(current_user_profile)
            .set((
                birthday.eq(new_profile.birthday),
                profile_image.eq(new_profile.profile_image),
            ))
            .execute(&mut conn)?;

        // Also edit the primary name

        // First We get the user_names of the account
        // We assume that user has at least primary name
        let name = UserName::belonging_to(account)
            .filter(primary_name.eq(true))
            .first::<UserName>(&mut conn)?;

        // Now we update it
        diesel::update(&name)
            .set((
                first_name.eq(new_profile.first_name),
                last_name.eq(new_profile.last_name),
            ))
            .execute(&mut conn)?;

        Ok("Edited")
    })
    .await
    .unwrap();

    edit_status
}
