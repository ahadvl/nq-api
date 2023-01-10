#[cfg(test)]
mod tests {
    use actix_web::{test, App};

    // #[test]
    // async fn get_surah() {
    //     let pool = Pool::builder()
    //         .build(establish_database_connection())
    //         .expect("Cant connect to db");

    //     let app = test::init_service(
    //         App::new()
    //             .app_data(web::Data::new(pool.clone()))
    //             .service(web::resource("/quran").route(web::get().to(quran))),
    //     )
    //     .await;

    //     let req = test::TestRequest::get()
    //         .uri("/quran?from=1&to=1")
    //         .to_request();

    //     let resp = test::call_service(&app, req).await;
    //     assert_eq!(resp.status(), StatusCode::OK);
    // }

    #[test]
    async fn send_bad_url() {
        let req = test::TestRequest::get()
            .uri("/quran?from=3333333&to=1")
            .to_request();

        let app = test::init_service(App::new()).await;

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 404)
    }
}
