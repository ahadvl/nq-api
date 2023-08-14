use crate::{models::Token, DbPool};
use actix_web::web;
use async_trait::async_trait;
use auth_n::token::{TokenChecker, TokenGenerator};
use diesel::prelude::*;

/// Returns the token selected
/// from database
#[derive(Clone)]
pub struct UserIdFromToken {
    db_pool: DbPool,
}

impl UserIdFromToken {
    pub fn new(db_pool: DbPool) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl TokenChecker<u32> for UserIdFromToken {
    async fn get_user_id(&self, request_token: &str) -> Option<u32> {
        use crate::schema::app_tokens::dsl::*;

        // Token as bytes
        let token_bytes: Vec<u8> = request_token.bytes().collect();

        let mut conn = self.db_pool.get().unwrap();

        let token = web::block(move || {
            // Hash the request token
            // Here we use tokengenerator
            // But we can just use sha2
            let mut token_generator = TokenGenerator::new(&token_bytes);
            token_generator.generate();

            // Selected hashed token from db
            let token = app_tokens
                .filter(token_hash.eq(token_generator.get_result().unwrap()))
                .load::<Token>(&mut conn)
                .unwrap();

            token
        })
        .await
        .unwrap();

        // Is there any token we found ?
        if token.is_empty() {
            return None;
        }

        let last_token = token.get(0).unwrap();

        // Return None for teminated token
        if last_token.terminated {
            return None;
        }

        Some(last_token.user_id as u32)
    }
}
