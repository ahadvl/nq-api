pub mod account {
    use crate::models::VerifyCode;
    use crate::schema::app_verify_codes::dsl::*;
    use crate::DbPool;
    use actix_web::{post, web, HttpResponse};
    use diesel::prelude::*;
    use rand::Rng;
    use serde::Deserialize;

    fn generate_random_code(min: u32, max: u32) -> u32 {
        let num: u32 = rand::thread_rng().gen_range(min..max);

        num
    }

    #[derive(Deserialize)]
    pub struct AccountInfo {
        email: Option<String>,
    }

    #[post("/account/sendCode")]
    pub async fn send_code(pool: web::Data<DbPool>, info: web::Json<AccountInfo>) -> HttpResponse {
        let random_code = generate_random_code(100000, 999999);

        let last_sended_code = web::block(move || {
            let mut conn = pool.get().unwrap();
            app_verify_codes
                .filter(email.eq(info.email.clone()))
                .order_by(created_at)
                .limit(1)
                .load::<VerifyCode>(&mut conn)
                .unwrap()
        })
        .await
        .unwrap();

        if last_sended_code.len() > 0 {
            let current_date = chrono::offset::Local::now();

            println!("{:?}", current_date);
            println!("{}", last_sended_code[0].created_at);
        }

        HttpResponse::Ok().body(format!("{}", random_code))
    }
}
