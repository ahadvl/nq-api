use super::{time_deference, MAX_RANDOM_CODE, MIN_RANDOM_CODE};
use crate::models::{Email, NewEmail, NewToken, NewUser, User, VerifyCode};
use crate::schema::app_emails;
use crate::{validate::validate, DbPool};
use actix_web::http::StatusCode;
use actix_web::{web, Error, HttpResponse};
use auth::token::TokenGenerator;
use diesel::prelude::*;
use rand::Rng;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Clone, Validate)]
pub struct VerifyCodeInfo {
    #[validate(email)]
    email: String,

    #[validate(range(min = "MIN_RANDOM_CODE", max = "MAX_RANDOM_CODE"))]
    code: i32,
}

enum UserStatus {
    /// This means user created at verifying email
    Created,

    /// This means user exists and just wants to get token (login)
    Exists,
}

impl UserStatus {
    /// Returns the status code of UserStatus
    pub fn as_status_code(&self) -> StatusCode {
        match *self {
            UserStatus::Created => StatusCode::CREATED,
            UserStatus::Exists => StatusCode::OK,
        }
    }
}

/// Verify verification code that sended to email
/// from /account/sendCode router
pub async fn verify(
    pool: web::Data<DbPool>,
    info: web::Json<VerifyCodeInfo>,
) -> Result<HttpResponse, Error> {
    use crate::schema::app_tokens;
    use crate::schema::app_users;
    use crate::schema::app_verify_codes::dsl::*;

    validate(&info.0)?;

    let result: (String, StatusCode) = web::block(move || {
        let mut conn = pool.get().unwrap();

        let last_sended_code = app_verify_codes
            .filter(email.eq(info.clone().email))
            .order(created_at.desc())
            .limit(1)
            .load::<VerifyCode>(&mut conn)
            .unwrap();

        if last_sended_code.is_empty() {
            return (
                "No coded sended to this email".to_string(),
                StatusCode::GONE,
            );
        }

        if last_sended_code[0].code != info.code {
            return ("Code is not correct".to_string(), StatusCode::OK);
        }

        if last_sended_code[0].status == *"used".to_string() {
            return ("Code is already used".to_string(), StatusCode::OK);
        }

        let diff = time_deference(last_sended_code[0].created_at);

        if diff.num_seconds() >= 70 {
            // status code 410 => Gone
            // The requested resource is no longer available at the server and no forwarding
            // address is known. This condition is expected to be considered permanent.

            return ("Code expired".to_string(), StatusCode::GONE);
        }

        // Everything is ok now change code status to used
        diesel::update(&last_sended_code[0])
            .set(status.eq("used".to_string()))
            .execute(&mut conn)
            .unwrap();

        // Check if user exists
        let user_email = app_emails::dsl::app_emails
            .filter(app_emails::dsl::email.eq(&info.email))
            .load::<Email>(&mut conn)
            .unwrap();

        // If we dont have user with request (email) then create it
        // else return it
        let (user, user_status): (User, UserStatus) = if user_email.is_empty() {
            let user = NewUser {
                username: &"".to_string(),
            };

            let new_user: User = diesel::insert_into(app_users::dsl::app_users)
                .values(&user)
                .get_result(&mut conn)
                .unwrap();

            let n_email = NewEmail {
                email: &info.email,
                user_id: &new_user.id,
                verified: true,
                primary: false,
                deleted: false,
            };

            let _new_email = diesel::insert_into(app_emails::dsl::app_emails)
                .values(&n_email)
                .execute(&mut conn)
                .unwrap();

            diesel::update(&new_user)
                .set(app_users::dsl::username.eq(format!("u{}", &new_user.id)))
                .execute(&mut conn)
                .unwrap();

            (new_user, UserStatus::Created)
        } else {
            let user = app_users::dsl::app_users
                .filter(app_users::dsl::id.eq(user_email.get(0).unwrap().user_id))
                .load::<User>(&mut conn)
                .unwrap();

            (user.get(0).unwrap().to_owned(), UserStatus::Exists)
        };

        // Some salts
        let user_id_as_string = user.id.to_string();
        let time_as_string = chrono::offset::Utc::now().timestamp().to_string();
        let mut random_bytes = rand::thread_rng().gen::<[u8; 32]>().to_vec();

        // source buffer for token
        let mut source = vec![];

        // append salts to the source
        source.append(&mut user_id_as_string.as_bytes().to_vec());
        source.append(&mut random_bytes);
        source.append(&mut time_as_string.as_bytes().to_vec());

        let mut token = TokenGenerator::new(&source);

        token.generate();

        let result = token.get_result().unwrap();

        // Hash the token itself
        let token_hash = {
            let result_bytes = result.as_bytes().to_vec();

            token.set_source(&result_bytes);
            token.generate();

            token.get_result().unwrap()
        };

        let new_token = NewToken {
            user_id: &user.id,
            token_hash: &token_hash,
        };

        // Save token to the Db
        diesel::insert_into(app_tokens::dsl::app_tokens)
            .values(&new_token)
            .execute(&mut conn)
            .unwrap();

        (result, user_status.as_status_code())
    })
    .await
    .unwrap();

    Ok(HttpResponse::build(result.1).body(result.0))
}
