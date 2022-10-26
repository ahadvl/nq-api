use super::time_deference;
use crate::email::EmailManager;
use crate::models::{NewVerifyCode, VerifyCode};
use crate::DbPool;
use actix_web::{post, web, HttpResponse};
use diesel::prelude::*;
use rand::Rng;
use serde::Deserialize;

const MIN_RANDOM_CODE: i32 = 100000;
const MAX_RANDOM_CODE: i32 = 999999;

pub fn generate_random_code(min: i32, max: i32) -> i32 {
    let num: i32 = rand::thread_rng().gen_range(min..max);

    num
}

#[derive(Deserialize, Clone)]
pub struct SendCodeInfo {
    email: String,
}

/// <data> -> Email,
/// Send Random generated code to user email
#[post("/account/sendCode")]
pub async fn send_code(
    pool: web::Data<DbPool>,
    emailer: web::Data<EmailManager>,
    info: web::Json<SendCodeInfo>,
) -> HttpResponse {
    use crate::schema::app_verify_codes::dsl::*;

    let random_code = generate_random_code(MIN_RANDOM_CODE, MAX_RANDOM_CODE);
    let mut conn = pool.get().unwrap();
    let info_copy = info.clone();

    let final_code: String = web::block(move || {
        // Get last sended code, order by created_at
        let last_sended_code = app_verify_codes
            .filter(email.eq(&info_copy.email))
            .order(created_at.desc())
            .limit(1)
            .load::<VerifyCode>(&mut conn)
            .unwrap();

        // Is there any code we sent ?
        if !last_sended_code.is_empty() {
            let diff = time_deference(last_sended_code[0].created_at.time());

            // Time deference between current date and last code created_at

            // Check if code not expired
            if diff.num_seconds() < 5 {
                return random_code.to_string();
            }
        }

        // Create new code
        let new_code = NewVerifyCode {
            code: &random_code,
            email: &info_copy.email,
            status: &"notUsed".to_string(),
        };

        // Insert code to app_verify_code table
        diesel::insert_into(app_verify_codes)
            .values(&new_code)
            .execute(&mut conn)
            .unwrap();

        random_code.to_string()
    })
    .await
    .unwrap();

    let result = emailer
        .send_email(
            &info.email,
            &"Verification Code",
            format!("Code: {}", final_code),
        )
        .await;

    match result {
        Ok(()) => HttpResponse::Ok().body("Code Sended."),
        Err(_error) => HttpResponse::InternalServerError().body("Cant send code."),
    }
}
