use actix_web::{guard::Guard, http::header};

struct TokenGuard(String);

impl Guard for TokenGuard {
    fn check(&self, ctx: &actix_web::guard::GuardContext<'_>) -> bool {
        let Some(request_token)= ctx.head().headers().get(header::AUTHORIZATION) else {
            return false;
        };

        self.0.as_bytes() == request_token.as_bytes()
    }
}

/// Creates a new TokenGuard object
pub fn token(perv_token: String) -> impl Guard {
    TokenGuard(perv_token)
}
