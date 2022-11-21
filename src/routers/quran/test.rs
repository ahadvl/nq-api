#[cfg(test)]
mod tests {
    use crate::establish_database_connection;
    use crate::models::QuranText;
    use crate::quran::*;
    use actix_web::{http::StatusCode, test, web, App};
    use diesel::r2d2::Pool;
    use std::rc::Rc;

    #[test]
    async fn get_surah() {
        let pool = Pool::builder()
            .build(establish_database_connection())
            .expect("Cant connect to db");

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(quran),
        )
        .await;

        let req = Rc::new(
            test::TestRequest::get()
                .uri("/quran?from=1&to=1")
                .to_request(),
        );

        let req1 = Rc::clone(&req);

        let resp = test::call_service(&app, Rc::try_unwrap(req).unwrap()).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let resp_body: Vec<QuranText> =
            test::call_and_read_body_json(&app, Rc::try_unwrap(req1).unwrap()).await;
        assert_eq!(resp_body.len(), 7);
    }

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
