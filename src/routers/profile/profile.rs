use actix_web::web::{self, ReqData};
use actix_web::Responder;
use diesel::prelude::*;

use crate::DbPool;

pub async fn view_profile(pool: web::Data<DbPool>, data: ReqData<u32>) -> impl Responder {
    use crate::models::UserProfile;
    use crate::schema::app_users::dsl::*;

    // Get userId from token Checker
    let user_id = data.into_inner();

    // select user form db
    // with user_id
    let user_profile = web::block(move || {
        let mut conn = pool.get().unwrap();

        let user = app_users
            .filter(id.eq(user_id as i32))
            .select((
                username,
                first_name,
                last_name,
                birthday,
                profile_image,
                email,
            ))
            .load::<UserProfile>(&mut conn)
            .unwrap();

        let last_user = user.get(0).unwrap();

        last_user.clone()
    })
    .await
    .unwrap();

    // Response with user as json
    web::Json(user_profile)
}
