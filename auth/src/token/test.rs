#[cfg(test)]
mod tests {
    use crate::token::{token, TokenGenerator};
    use actix_web::guard::Guard;
    use actix_web::http::header;
    use actix_web::test::TestRequest;

    #[test]
    fn test_token_generator() {
        let source: Vec<u8> = vec![1, 2, 3];

        let mut token_generator = TokenGenerator::new(&source);

        token_generator.generate();

        assert_eq!(token_generator.get_result().unwrap().len(), 64);
    }

    #[test]
    fn test_token_guard() {
        let get_req = TestRequest::get()
            .insert_header((header::AUTHORIZATION, "secret-token"))
            .to_srv_request();

        let guard = token(String::from("secret-token"));

        assert!(guard.check(&get_req.guard_ctx()));

        let guard = token(String::from("-secret-whoops($#@!)"));

        assert!(!guard.check(&get_req.guard_ctx()));
    }
}
