use super::time_deference;
use crate::models::VerifyCode;
use crate::schema::app_verify_codes::dsl::*;
use crate::DbPool;
use actix_web::http::StatusCode;
use actix_web::{post, web, HttpResponse};
use diesel::prelude::*;
use serde::Deserialize;
use std::fmt::Write;

#[derive(Deserialize, Clone)]
pub struct VerifyCodeInfo {
    email: String,
    code: i32,
}

// TODO:
pub struct Token {
    result_source: [u8; 32],
}

/// Verify verification code that sended to email
/// from /account/sendCode router
#[post("/account/verify")]
pub async fn verify(pool: web::Data<DbPool>, info: web::Json<VerifyCodeInfo>) -> HttpResponse {
    let mut conn = pool.get().unwrap();

    // Ok (token) , Err(Message, status_code)
    let token: Result<Token, (String, StatusCode)> = web::block(move || {
        let last_sended_code = app_verify_codes
            .filter(email.eq(info.clone().email))
            .order(created_at.desc())
            .limit(1)
            .load::<VerifyCode>(&mut conn)
            .unwrap();

        if last_sended_code.len() <= 0 {
            return Err(("Code is not valid".to_string(), StatusCode::NOT_FOUND));
        }

        if last_sended_code[0].status == "used".to_string() {
            return Err(("Code is not valid".to_string(), StatusCode::NOT_FOUND));
        }

        let diff = time_deference(last_sended_code[0].created_at.time());

        if diff.num_seconds() >= 70 {
            // status code 410 => Gone
            // The requested resource is no longer available at the server and no forwarding
            // address is known. This condition is expected to be considered permanent.

            return Err(("Code expired".to_string(), StatusCode::GONE));
        }

        if last_sended_code[0].code != info.code {
            return Err(("Code is not correct".to_string(), StatusCode::NOT_FOUND));
        }

        // Everything is ok now change code status to used
        diesel::update(&last_sended_code[0])
            .set(status.eq("used".to_string()))
            .execute(&mut conn)
            .unwrap();

        // TODO: Create new user , Grab id and create Token

        Ok(Token {
            result_source: [0; 32],
        })
    })
    .await
    .unwrap();

    let response = match token {
        Ok(token) => {
            let mut as_string = String::with_capacity(2 * 32);
            for byte in token.result_source {
                write!(as_string, "{:02X}", byte).unwrap();
            }

            HttpResponse::Ok().body(as_string)
        }

        Err(error) => HttpResponse::build(error.1).body(error.0),
    };

    response
}
