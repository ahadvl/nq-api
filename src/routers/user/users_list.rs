use crate::user::FullUserProfile;
use crate::{error::RouterError, DbPool};
use actix_web::web;

pub async fn users_list(
    pool: web::Data<DbPool>,
) -> Result<web::Json<Vec<FullUserProfile>>, RouterError> {
    use crate::schema::app_users::dsl::app_users;

    let users_list = web::block(move || {
    }).await.unwrap();

    todo!();
}
