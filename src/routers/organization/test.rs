#[cfg(test)]
mod tests {
    use crate::routers::organization;
    use crate::routers::organization::new_organization_info::NewOrgInfo;
    use crate::test::Test;
    use crate::{establish_database_connection, run_migrations};
    use actix_web::body::{BodySize, MessageBody};
    use actix_web::test;
    use actix_web::{web, App};
    use diesel::r2d2::Pool;

    pub fn add_org_request() -> test::TestRequest {
        test::TestRequest::post()
            .uri("/organization")
            .set_json(NewOrgInfo::test())
    }

    #[test]
    pub async fn add_new_org() {
        let pg_manager = establish_database_connection();

        let pool = Pool::builder()
            .build(pg_manager)
            .expect("Failed to create pool.");

        let mut conn = pool.get().unwrap();

        run_migrations(&mut conn).unwrap();

        let app =
            test::init_service(App::new().app_data(web::Data::new(pool.clone())).service(
                web::resource("/organization").route(web::post().to(organization::add::add)),
            ))
            .await;

        let res = test::call_service(&app, add_org_request().to_request()).await;

        assert_eq!(res.status(), 200);
    }

    #[test]
    pub async fn view_org() {
        let pg_manager = establish_database_connection();

        let pool = Pool::builder()
            .build(pg_manager)
            .expect("Failed to create pool.");

        let mut conn = pool.get().unwrap();

        run_migrations(&mut conn).unwrap();

        let app = test::init_service(
            App::new().app_data(web::Data::new(pool.clone())).service(
                web::scope("/organization")
                    .route("", web::post().to(organization::add::add))
                    .route("/{org_id}", web::get().to(organization::view::view)),
            ),
        )
        .await;

        // first we add a new org
        let new_org_res = test::call_service(&app, add_org_request().to_request()).await;

        assert_eq!(new_org_res.status(), 200);

        // Now create request to view the org
        let get_org = test::TestRequest::get().uri("/organization/1").to_request();

        let new_org_res = test::call_service(&app, get_org).await;

        assert_eq!(new_org_res.status(), 200);

        let organization = new_org_res.response().body();

        assert_eq!(organization.size(), BodySize::Sized(209));
    }
}
