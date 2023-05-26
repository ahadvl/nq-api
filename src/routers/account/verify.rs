use super::{time_deference, MAX_RANDOM_CODE, MIN_RANDOM_CODE};
use crate::error::RouterError;
use crate::models::{Account, Email, NewAccount, NewEmail, NewToken, NewUser, User, VerifyCode};
use crate::schema::app_emails;
use crate::{validate::validate, DbPool};
use actix_web::web;
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

        let Ok(last_sended_code) = app_verify_codes
            .filter(email.eq(info.clone().email))
            .order(created_at.desc())
            .limit(1)
            .load::<VerifyCode>(&mut conn)
            else {
                return Err(RouterError::InternalError);
            };

        let Some(last_sended_code) = last_sended_code.get(0) else {
            return Err(RouterError::Gone(
                "No coded sended to this email".to_string(),
            ));
        };

        // The code is not correct
        if last_sended_code.code != info.code {
            return Ok("Code is not correct".to_string());
        }

        // The code is already used
        if last_sended_code.status == *"used".to_string() {
            return Ok("Code is already used".to_string());
        }

        // Get the time difference for expireation check
        let diff = time_deference(last_sended_code.created_at);

        if diff.num_seconds() >= 70 {
            // status code 410 => Gone
            // The requested resource is no longer available at the server and no forwarding
            // address is known. This condition is expected to be considered permanent.

            return Err(RouterError::Gone("Code expired".to_string()));
        }

        // Everything is ok now change code status to used
        let Ok(_) = diesel::update(&last_sended_code)
            .set(status.eq("used".to_string()))
            .execute(&mut conn) else {
                return Err(RouterError::InternalError);
            };

        // Check if user exists
        let Ok(user_email) = app_emails::dsl::app_emails
            .filter(app_emails::dsl::email.eq(&info.email))
            .load::<Email>(&mut conn)
            else {
                return Err(RouterError::InternalError);
            };

        // If we dont have user with request (email) then create it
        // else return it
        let user: User = if user_email.is_empty() {
            // Create a new account
            let Ok(new_account) = NewAccount {
                username: &String::from(""),
                account_type: &String::from("user"),
            }
            .insert_into(app_accounts::dsl::app_accounts)
            .get_result::<Account>(&mut conn)
            else {
                return Err(RouterError::InternalError)
            };

            let Ok(new_user)= NewUser {
                account_id: new_account.id,
                language: None,
            }
            .insert_into(app_users::dsl::app_users)
            .get_result::<User>(&mut conn)
            else {
                return Err(RouterError::InternalError)
            };

            let Ok(_) = NewEmail {
                email: &info.email,
                account_id: new_account.id,
                verified: true,
                primary: false,
                deleted: false,
            }
            .insert_into(app_emails::dsl::app_emails)
            .execute(&mut conn) else {
                return Err(RouterError::InternalError)
            };

            // Update the account and set the user name to the
            // u{the new account id}
            let Ok(_) = diesel::update(&new_account)
                .set(app_accounts::dsl::username.eq(format!("u{}", &new_account.id)))
                .execute(&mut conn)
                else {
                    return Err(RouterError::InternalError);
                };

            new_user
        } else {
            let Ok(user) = app_users::dsl::app_users
                .filter(app_users::dsl::account_id.eq(user_email.get(0).unwrap().account_id))
                .load::<User>(&mut conn)
                else {
                    return Err(RouterError::InternalError);
                };

            let Some(user) = user.get(0) else {
                return Err(RouterError::InternalError);
            };

            user.to_owned()
        };

        // Some salts
        let user_account_id_as_string = user.account_id.to_string();
        let time_as_string = chrono::offset::Utc::now().timestamp().to_string();
        let mut random_bytes = rand::thread_rng().gen::<[u8; 32]>().to_vec();

        // source buffer for token
        let mut source = vec![];

        // append salts to the source
        source.append(&mut user_account_id_as_string.as_bytes().to_vec());
        source.append(&mut random_bytes);
        source.append(&mut time_as_string.as_bytes().to_vec());

        let mut token = TokenGenerator::new(&source);

        token.generate();

        let Some(result) = token.get_result() else {
            return Err(RouterError::InternalError);
        };

        // Hash the token itself
        let token_hash = {
            let result_bytes = result.as_bytes().to_vec();

            token.set_source(&result_bytes);
            token.generate();

            token.get_result().unwrap()
        };

        let new_token = NewToken {
            account_id: &user.account_id,
            token_hash: &token_hash,
        };

        // Save token to the Db
        let Ok(_) = diesel::insert_into(app_tokens::dsl::app_tokens)
            .values(&new_token)
            .execute(&mut conn) else {
                return Err(RouterError::InternalError);
            };

        Ok(result)
    })
    .await
    .unwrap();

    result
}
