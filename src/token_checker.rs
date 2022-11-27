use crate::{models::Token, DbPool};
use auth::token::{TokenChecker, TokenGenerator};
use diesel::prelude::*;

/// Returns the token selected
/// from database
#[derive(Clone)]
pub struct TokenFromDatabase {
    pool: DbPool,
}

impl TokenFromDatabase {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl TokenChecker for TokenFromDatabase {
    fn check_token(&self, request_token: &str) -> Option<u32> {
        use crate::schema::app_tokens::dsl::*;

        let token_bytes: Vec<u8> = request_token.bytes().collect();

        // Hash the request token
        // Here we use tokengenerator
        // But we can just use sha2
        let mut token_generator = TokenGenerator::new(&token_bytes);
        token_generator.generate();

        let mut conn = self.pool.get().unwrap();

        // Selected hashed token from db
        let token = app_tokens
            .filter(token_hash.eq(token_generator.get_result().unwrap()))
            .load::<Token>(&mut conn)
            .unwrap();

        if token.is_empty() {
            return None;
        }

        let last_token = token.get(0).unwrap();

        Some(last_token.user_id as u32)
    }
}
