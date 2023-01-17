use actix_web::{
    error,
    web::{self, ReqData},
    Responder,
};
use diesel::{prelude::*, result::Error};

use crate::{
    models::{Account, User, UserProfile},
    DbPool,
};

/// Edit the profile
/// wants a new profile and token
pub async fn edit_profile(
    data: ReqData<u32>,
    pool: web::Data<DbPool>,
    new_profile: web::Json<UserProfile>,
) -> Result<impl Responder, error::Error> {
    use crate::schema::app_accounts::dsl::{app_accounts, id as acc_id, username};
    use crate::schema::app_users::dsl::*;

    let user_id = data.into_inner();
    let new_profile = new_profile.into_inner();

    let edit_status: Result<(), Error> = web::block(move || {
        let mut conn = pool.get().unwrap();

        // First find the org from id
        let account = app_accounts
            .filter(acc_id.eq(user_id as i32))
            .load::<Account>(&mut conn)?;

        let account = account.get(0).unwrap();

        let user = User::belonging_to(account).load::<User>(&mut conn)?;

        let current_user_profile = user.get(0).unwrap();

        // Now update the account username
        diesel::update(account)
            .set(username.eq(new_profile.username))
            .execute(&mut conn)?;

        // And update the other data
        diesel::update(current_user_profile)
            .set((
                first_name.eq(new_profile.first_name),
                last_name.eq(new_profile.last_name),
                birthday.eq(new_profile.birthday),
                profile_image.eq(new_profile.profile_image),
            ))
            .execute(&mut conn)?;

        Ok(())
    })
    .await
    .unwrap();

    match edit_status {
        Ok(()) => Ok("edited"),
        Err(_error) => Err(error::ErrorInternalServerError("Cant edit the profile")),
    }
}
