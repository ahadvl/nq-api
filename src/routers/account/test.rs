#[cfg(test)]
mod tests {
    use crate::{
        create_emailer, establish_database_connection,
        routers::account::{
            send_code::{send_code, SendCodeInfo},
            verify::TokenGenerator,
        },
        test::Test,
    };
    use actix_web::{
        body::{BodySize, MessageBody},
        http::StatusCode,
        test, web, App,
    };
    use diesel::r2d2::Pool;
    use rand::Rng;

    #[test]
    pub async fn send_code_test() {
        let pg_manager = establish_database_connection();

        let pool = Pool::builder()
            .build(pg_manager)
            .expect("Failed to create pool.");

        let mailer = create_emailer();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .app_data(web::Data::new(mailer.clone()))
                .service(send_code),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/account/sendCode")
            .set_json(SendCodeInfo::test())
            .to_request();

        let res = test::call_service(&app, req).await;

        assert_eq!(res.status(), StatusCode::OK);

        assert_eq!(res.response().body().size(), BodySize::Sized(11));
    }

    const EXAMPLE_USER_ID: u8 = 22;
    const RANDOM_BYTES_SIZE: usize = 32;

    #[test]
    pub async fn test_token_generator() {
        let user_id_as_string = EXAMPLE_USER_ID.to_string();
        let time_as_string = chrono::offset::Utc::now().timestamp().to_string();
        let mut random_bytes = rand::thread_rng().gen::<[u8; RANDOM_BYTES_SIZE]>().to_vec();

        assert_eq!(random_bytes.len(), RANDOM_BYTES_SIZE);

        let mut source: Vec<u8> = vec![];
        source.append(&mut user_id_as_string.as_bytes().to_vec());
        source.append(&mut random_bytes);
        source.append(&mut time_as_string.as_bytes().to_vec());

        assert_eq!(
            source.len(),
            user_id_as_string.len() + time_as_string.len() + RANDOM_BYTES_SIZE
        );

        let mut token_generator = TokenGenerator::new(&source);

        token_generator.generate();

        assert_eq!(token_generator.get_result().unwrap().len(), 64);
    }

    /// TODO:
    ///       write test for verify router,
    ///       I have no idea how to test this router.
    ///
    pub async fn _verify() {
        todo!()
    }
}
