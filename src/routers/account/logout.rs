use crate::DbPool;
use actix_web::{
    http::StatusCode,
    web::{self, ReqData},
    HttpResponse, Responder,
};
use diesel::prelude::*;

/// This will teminate the user token
pub async fn logout(pool: web::Data<DbPool>, data: ReqData<u32>) -> impl Responder {
    use crate::models::Token;
    use crate::schema::app_tokens::dsl::*;

    let req_user_id = data.into_inner();

    // String -> Message
    let result: Result<(), (&str, StatusCode)> = web::block(move || {
        let mut conn = pool.get().unwrap();

        // Get the latest token
        let tokens: Vec<Token> = app_tokens
            .filter(user_id.eq(req_user_id as i32))
            .order(created_at.desc())
            .limit(1)
            .load::<Token>(&mut conn)
            .unwrap();

        // Get THE token
        let token = tokens.get(0).unwrap();

        // Now teminate the token
        // Set the terminated to true
        // And set request id to the terminated_by_id
        // This may change.
        diesel::update(token)
            .set((terminated.eq(true), terminated_by_id.eq(req_user_id as i32)))
            .execute(&mut conn)
            .unwrap();

        Ok(())
    })
    .await
    .unwrap();

    match result {
        // Users token Successfuly terminated
        Ok(()) => HttpResponse::build(StatusCode::OK).body("Ok"),

        // Build Response with message (error.0) and status (error.1)
        Err(error) => HttpResponse::build(error.1).body(error.0),
    }
}
