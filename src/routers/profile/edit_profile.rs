use actix_web::web::{self, ReqData};
use diesel::prelude::*;

use crate::{
    error::RouterError,
    models::{Account, User, UserProfile},
    DbPool,
};

// TODO: Create a router for handling names
/// Edit the profile
/// wants a new profile and token
pub async fn edit_profile(
    data: ReqData<u32>,
    pool: web::Data<DbPool>,
    new_profile: web::Json<UserProfile>,
) -> Result<String, RouterError> {
    use crate::schema::app_accounts::dsl::{app_accounts, id as acc_id, username};
    use crate::schema::app_users::dsl::*;

    let user_id = data.into_inner();
    let new_profile = new_profile.into_inner();

    let edit_status: Result<String, RouterError> = web::block(move || {
        let mut conn = pool.get().unwrap();

        // First find the org from id
        let Ok(account) = app_accounts
            .filter(acc_id.eq(user_id as i32))
            .load::<Account>(&mut conn)
            else {
                return Err(RouterError::InternalError);
            };

        let Some(account) = account.get(0) else {
            return Err(RouterError::NotFound("Account not found".to_string()));
        };

        let Ok(user) = User::belonging_to(account).load::<User>(&mut conn) else {
            return Err(RouterError::InternalError);
        };

        let Some(current_user_profile) = user.get(0) else {
            return Err(RouterError::NotFound("user not found".to_string()));
        };

        // Now update the account username
        let Ok(_) = diesel::update(account)
            .set(username.eq(new_profile.username))
            .execute(&mut conn) else {
                return Err(RouterError::InternalError);
            };

        // And update the other data
        let Ok(_) = diesel::update(current_user_profile)
            .set((
                // first_name.eq(new_profile.first_name),
                // last_name.eq(new_profile.last_name),
                birthday.eq(new_profile.birthday),
                profile_image.eq(new_profile.profile_image),
            ))
            .execute(&mut conn)
            else {
                return Err(RouterError::InternalError);
            };

        Ok("Edited".to_string())
    })
    .await
    .unwrap();

    edit_status
}
