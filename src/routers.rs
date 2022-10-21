pub mod account {
    use crate::models::{NewVerifyCode, VerifyCode};
    use crate::schema::app_verify_code::dsl::*;
    use crate::DbPool;
    use actix_web::{post, web, HttpResponse};
    use diesel::prelude::*;
    use rand::Rng;
    use serde::Deserialize;

    const MIN_RANDOM_CODE: i32 = 100000;
    const MAX_RANDOM_CODE: i32 = 999999;

    fn generate_random_code(min: i32, max: i32) -> i32 {
        let num: i32 = rand::thread_rng().gen_range(min..max);

        num
    }

    #[derive(Deserialize, Clone)]
    pub struct SendCodeInfo {
        email: String,
    }

    #[post("/account/sendCode")]
    pub async fn send_code(pool: web::Data<DbPool>, info: web::Json<SendCodeInfo>) -> HttpResponse {
        let random_code = generate_random_code(MIN_RANDOM_CODE, MAX_RANDOM_CODE);
        let mut conn = pool.get().unwrap();

        let response = web::block(move || {
            let last_sended_code = app_verify_code
                .filter(email.eq(&info.email))
                .order_by(created_at)
                .limit(1)
                .load::<VerifyCode>(&mut conn)
                .unwrap();

            if last_sended_code.len() > 0 {
                let current_date = chrono::offset::Local::now().timestamp();
                let last_code_date = last_sended_code[0].created_at.timestamp();

                if (current_date - last_code_date) < 5000 {
                    // TODO: Send same code here, do not create a new code
                    return "Code sended".to_string();
                }
            }

            let new_code = NewVerifyCode {
                code: &random_code,
                email: &info.email,
                status: &"notUsed".to_string(),
            };

            diesel::insert_into(app_verify_code)
                .values(&new_code)
                .execute(&mut conn)
                .unwrap();

            // TODO: Send code Here

            return "Code sended".to_string();
        })
        .await
        .unwrap();

        HttpResponse::Ok().body(response)
    }

    pub async fn verify(pool: web::Data<DbPool>, info: web::Json<AccountInfo>) -> HttpResponse {
        let mut conn = pool.get().unwrap();

        web::block(move || {})
    }
}
