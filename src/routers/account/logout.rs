use crate::error::RouterError;
use crate::DbPool;
use actix_web::web::{self, ReqData};
use diesel::prelude::*;

/// This will teminate the user token
pub async fn logout(pool: web::Data<DbPool>, data: ReqData<u32>) -> Result<String, RouterError> {
    use crate::models::Token;
    use crate::schema::app_tokens::dsl::*;

    let req_account_id = data.into_inner();

    // String -> Message
    let result = web::block(move || {
        let mut conn = pool.get().unwrap();

        // Get the latest token
        let Ok(tokens) = app_tokens
            .filter(account_id.eq(req_account_id  as i32))
            .order(created_at.desc())
            .limit(1)
            .load::<Token>(&mut conn) else {
                return Err(RouterError::InternalError)
            };

        // Get THE token
        let Some(token) = tokens.get(0) else {
            return Err(RouterError::NotFound("Token not found".to_string()));
        };

        // Now teminate the token
        // Set the terminated to true
        // And set request id to the terminated_by_id
        // This may change.
        let Ok(_) =  diesel::update(token)
            .set((terminated.eq(true), terminated_by_id.eq(req_account_id as i32)))
            .execute(&mut conn) else {
                return Err(RouterError::InternalError)
            };

        Ok("Logged Out".to_string())
    })
    .await
    .unwrap()?;

    Ok(result)
}
