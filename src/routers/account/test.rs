#[cfg(test)]
mod tests {
    use crate::{
        create_emailer, establish_database_connection,
        routers::account::send_code::{send_code, SendCodeInfo},
        run_migrations,
        test::Test,
    };
    use actix_web::{
        body::{BodySize, MessageBody},
        http::StatusCode,
        test, web, App,
    };
    use diesel::r2d2::Pool;

    #[test]
    pub async fn send_code_test() {
        let pg_manager = establish_database_connection();

        let pool = Pool::builder()
            .build(pg_manager)
            .expect("Failed to create pool.");

        run_migrations(&mut pool.get().unwrap()).unwrap();

        let mailer = create_emailer();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .app_data(web::Data::new(mailer.clone()))
                .service(web::resource("/account/sendCode").route(web::post().to(send_code))),
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

    /// TODO:
    ///       write test for verify router,
    ///       I have no idea how to test this router.
    ///
    pub async fn _verify() {
        todo!()
    }
}
