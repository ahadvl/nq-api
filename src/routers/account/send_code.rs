use super::{time_deference, MAX_RANDOM_CODE, MIN_RANDOM_CODE};
use crate::email::EmailManager;
use crate::models::{NewVerifyCode, VerifyCode};
use crate::validate::validate;
use crate::DbPool;
use actix_web::{error, post, web, Error, HttpResponse};
use diesel::prelude::*;
use rand::Rng;
use serde::Deserialize;
use validator::Validate;

/// Generate random code with range (min, max)
pub fn generate_random_code(min: i32, max: i32) -> i32 {
    let num: i32 = rand::thread_rng().gen_range(min..max);

    num
}

enum SendCodeStatus {
    /// This means we need to send code
    /// to email
    SendCode(String),

    /// This means we already sended code
    /// in past 10 seconds
    AlreadySent,
}

#[derive(Deserialize, Clone, Validate)]
pub struct SendCodeInfo {
    #[validate(email)]
    email: String,
}

/// <data> -> Email,
/// Send Random generated code to user email
#[post("/account/sendCode")]
pub async fn send_code(
    pool: web::Data<DbPool>,
    emailer: web::Data<EmailManager>,
    info: web::Json<SendCodeInfo>,
) -> Result<HttpResponse, Error> {
    use crate::schema::app_verify_codes::dsl::*;

    validate(&info.0)?;

    let info_copy = info.clone();

    let send_status: SendCodeStatus = web::block(move || {
        let random_code = generate_random_code(MIN_RANDOM_CODE, MAX_RANDOM_CODE);
        let mut conn = pool.get().unwrap();

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

            // Check if code not expired
            if diff.num_seconds() < 10 {
                // TODO: Dont send code to email
                // return code is sended.
                return SendCodeStatus::AlreadySent;
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

        SendCodeStatus::SendCode(random_code.to_string())
    })
    .await
    .unwrap();

    match send_status {
        SendCodeStatus::SendCode(new_code) => {
            let result = emailer
                .send_email(
                    &info.email,
                    "Verification Code",
                    format!("Code: {}", new_code),
                )
                .await;

            match result {
                Ok(()) => Ok(HttpResponse::Ok().body("Code sended")),
                // TODO: maybe its not good idea to send error here
                Err(error) => Err(error::ErrorInternalServerError(error)),
            }
        }
        SendCodeStatus::AlreadySent => Ok(HttpResponse::Ok().body("Already sent")),
    }
}
