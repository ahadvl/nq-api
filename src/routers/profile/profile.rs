use actix_web::web::{self, ReqData};
use actix_web::Responder;
use diesel::prelude::*;

use crate::DbPool;

pub async fn view_profile(pool: web::Data<DbPool>, data: ReqData<u32>) -> impl Responder {
    use crate::models::User;
    use crate::schema::app_users::dsl::*;

    let user_id = data.into_inner();

    let user_profile = web::block(move || {
        let mut conn = pool.get().unwrap();

        let user = app_users
            .filter(id.eq(user_id as i32))
            .load::<User>(&mut conn)
            .unwrap();

        let last_user = user.get(0).unwrap();

        last_user.clone()
    })
    .await
    .unwrap();

    web::Json(user_profile)
}
