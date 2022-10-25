use crate::models::{NewUser, NewVerifyCode, User, VerifyCode};
use crate::DbPool;
use actix_web::http::StatusCode;
use actix_web::{post, web, HttpResponse};
use chrono::{Duration, NaiveTime};
use diesel::prelude::*;
use rand::Rng;
use serde::Deserialize;
use sha2::{Digest, Sha256};

/// Get deference between Current time and past_time
pub fn time_deference(past_time: NaiveTime) -> Duration {
    let current_date = chrono::offset::Utc::now().time();

    current_date - past_time
}

const MIN_RANDOM_CODE: i32 = 100000;
const MAX_RANDOM_CODE: i32 = 999999;

fn generate_random_code(min: i32, max: i32) -> i32 {
    let num: i32 = rand::thread_rng().gen_range(min..max);

    num
}

#[derive(Clone)]
pub(self) struct Token<'a> {
    /// Target data
    source: &'a Vec<u8>,

    /// Final Generated Token
    result: Option<String>,
}

impl<'a> Token<'a> {
    /// Creates a new Token object
    pub fn new(source: &'a Vec<u8>) -> Self {
        Self {
            source,
            result: None,
        }
    }

    /// Generates the final hash and set to result
    pub fn generate(&mut self) {
        let mut hasher = Sha256::new();

        hasher.update(self.source);

        self.result = Some(format!("{:x}", hasher.finalize()));
    }
}

#[derive(Deserialize, Clone)]
pub struct SendCodeInfo {
    email: String,
}

#[derive(Deserialize, Clone)]
pub struct VerifyCodeInfo {
    email: String,
    code: i32,
}

/// <data> -> Email,
/// Send Random generated code to user email
#[post("/account/sendCode")]
pub async fn send_code(pool: web::Data<DbPool>, info: web::Json<SendCodeInfo>) -> HttpResponse {
    use crate::schema::app_verify_codes::dsl::*;

    let random_code = generate_random_code(MIN_RANDOM_CODE, MAX_RANDOM_CODE);
    let mut conn = pool.get().unwrap();

    let result_msg = web::block(move || {
        // Get last sended code, order by created_at
        let last_sended_code = app_verify_codes
            .filter(email.eq(&info.email))
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
                // TODO: Send same code here, do not create a new code
                return "Code sended :)".to_string();
            }
        }

        // Create new code
        let new_code = NewVerifyCode {
            code: &random_code,
            email: &info.email,
            status: &"notUsed".to_string(),
        };

        // Insert code to app_verify_code table
        diesel::insert_into(app_verify_codes)
            .values(&new_code)
            .execute(&mut conn)
            .unwrap();

        // TODO: Send code here.
        // (email)

        "Code sended".to_string()
    })
    .await
    .unwrap();

    HttpResponse::Ok().body(result_msg)
}

/// Verify verification code that sended to email
/// from /account/sendCode router
#[post("/account/verify")]
pub async fn verify(pool: web::Data<DbPool>, info: web::Json<VerifyCodeInfo>) -> HttpResponse {
    use crate::schema::app_users;
    use crate::schema::app_verify_codes::dsl::*;

    let mut conn = pool.get().unwrap();

    // Ok (token) , Err(Message, status_code)
    let token_as_string: Result<String, (String, StatusCode)> = web::block(move || {
        let last_sended_code = app_verify_codes
            .filter(email.eq(info.clone().email))
            .order(created_at.desc())
            .limit(1)
            .load::<VerifyCode>(&mut conn)
            .unwrap();

        if last_sended_code.is_empty() {
            return Err(("Code is not valid".to_string(), StatusCode::NOT_FOUND));
        }

        if last_sended_code[0].status == *"used".to_string() {
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

        // Check if user exists
        let user_from_db = app_users::dsl::app_users
            .filter(app_users::dsl::email.eq(&info.email))
            .load::<User>(&mut conn)
            .unwrap();

        // If we dont have user with request (email) then create it
        // else return it
        let user: User = if user_from_db.is_empty() {
            let user = NewUser {
                email: &info.email,
                username: &"".to_string(),
            };

            let new_user: User = diesel::insert_into(app_users::dsl::app_users)
                .values(&user)
                .get_result(&mut conn)
                .unwrap();

            diesel::update(app_users::dsl::app_users)
                .set(app_users::dsl::username.eq(format!("u{}", &new_user.id)))
                .execute(&mut conn)
                .unwrap();

            new_user
        } else {
            let u = user_from_db.get(0).unwrap();

            u.clone()
        };

        // Some salts
        let user_id_as_string = user.id.to_string();
        let time_as_string = chrono::offset::Utc::now().timestamp().to_string();
        let mut random_bytes = rand::thread_rng().gen::<[u8; 32]>().to_vec();

        // source buffer for token
        let mut source = vec![];

        // append slats to the source
        source.append(&mut user_id_as_string.as_bytes().to_vec());
        source.append(&mut random_bytes);
        source.append(&mut time_as_string.as_bytes().to_vec());

        let mut token = Token::new(&source);

        token.generate();

        let result = token.result.unwrap();

        Ok(result)
    })
    .await
    .unwrap();

    match token_as_string {
        Ok(token) => HttpResponse::Ok().body(token),

        Err(error) => HttpResponse::build(error.1).body(error.0),
    }
}
