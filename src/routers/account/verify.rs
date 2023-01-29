use super::{time_deference, MAX_RANDOM_CODE, MIN_RANDOM_CODE};
use crate::error::RouterError;
use crate::models::{Account, Email, NewAccount, NewEmail, NewToken, NewUser, User, VerifyCode};
use crate::schema::app_emails;
use crate::{validate::validate, DbPool};
use actix_web::{web};
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

/// Verify verification code that sended to email
/// from /account/sendCode router
pub async fn verify(
    pool: web::Data<DbPool>,
    info: web::Json<VerifyCodeInfo>,
) -> Result<String, RouterError> {
    use crate::schema::app_accounts;
    use crate::schema::app_tokens;
    use crate::schema::app_users;
    use crate::schema::app_verify_codes::dsl::*;

    validate(&info.0)?;

    let result: Result<String, RouterError> = web::block(move || {
        let mut conn = pool.get().unwrap();

        let last_sended_code = app_verify_codes
            .filter(email.eq(info.clone().email))
            .order(created_at.desc())
            .limit(1)
            .load::<VerifyCode>(&mut conn)
            .unwrap();

        if last_sended_code.is_empty() {
            return Err(RouterError::Gone(
                "No coded sended to this email".to_string(),
            ));
        }

        if last_sended_code[0].code != info.code {
            return Ok("Code is not correct".to_string());
        }

        if last_sended_code[0].status == *"used".to_string() {
            return Ok("Code is already used".to_string());
        }

        let diff = time_deference(last_sended_code[0].created_at);

        if diff.num_seconds() >= 70 {
            // status code 410 => Gone
            // The requested resource is no longer available at the server and no forwarding
            // address is known. This condition is expected to be considered permanent.

            return Err(RouterError::Gone("Code expired".to_string()));
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
        let user: User = if user_email.is_empty() {
            let new_account: Account = NewAccount {
                username: &String::from(""),
                account_type: &String::from("user"),
            }
            .insert_into(app_accounts::dsl::app_accounts)
            .get_result(&mut conn)
            .unwrap();

            let new_user = NewUser {
                account_id: new_account.id,
            }
            .insert_into(app_users::dsl::app_users)
            .get_result(&mut conn)
            .unwrap();

            let _new_email = NewEmail {
                email: &info.email,
                account_id: new_account.id,
                verified: true,
                primary: false,
                deleted: false,
            }
            .insert_into(app_emails::dsl::app_emails)
            .execute(&mut conn);

            diesel::update(&new_account)
                .set(app_accounts::dsl::username.eq(format!("u{}", &new_account.id)))
                .execute(&mut conn)
                .unwrap();

            new_user
        } else {
            let user = app_users::dsl::app_users
                .filter(app_users::dsl::account_id.eq(user_email.get(0).unwrap().account_id))
                .load::<User>(&mut conn)
                .unwrap();

            user.get(0).unwrap().to_owned()
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

        Ok(result)
    })
    .await
    .unwrap();

    result
}
