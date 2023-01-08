use actix_web::web::{self, ReqData};
use actix_web::Responder;
use diesel::prelude::*;

use crate::models::{Account, User};
use crate::DbPool;

pub async fn view_profile(pool: web::Data<DbPool>, data: ReqData<u32>) -> impl Responder {
    use crate::schema::app_accounts::dsl::{app_accounts, id as id_from_accounts};

    // Get userId from token Checker
    let acc_id = data.into_inner();

    // select user form db
    // with user_id
    let user_profile = web::block(move || {
        let mut conn = pool.get().unwrap();

        let account = app_accounts
            .filter(id_from_accounts.eq(acc_id as i32))
            .load::<Account>(&mut conn)
            .unwrap();

        let account_user = User::belonging_to(account.get(0).unwrap())
            .load::<User>(&mut conn)
            .unwrap();

        account_user
    })
    .await
    .unwrap();

    // Response with user as json
    web::Json(user_profile)
}
