#[cfg(test)]
mod tests {
    use crate::models::{Account, NewUser};
    use crate::schema::{app_accounts::dsl::*, app_users};
    use crate::test::rollback_db;
    use crate::{
        establish_database_connection, models::NewAccount, routers::profile::profile,
        run_migrations,
    };
    use actix_web::test;
    use actix_web::{web, App};
    use diesel::prelude::*;
    use diesel::r2d2::Pool;

    // TODO: FIX issue
    #[test]
    pub async fn get_profile() {
        let pg_manager = establish_database_connection();

        let pool = Pool::builder()
            .build(pg_manager)
            .expect("Failed to create pool.");

        let mut conn = pool.get().unwrap();

        run_migrations(&mut pool.get().unwrap()).unwrap();

        // First create a account
        let acc: Account = NewAccount {
            account_type: &String::from("user"),
            username: &String::from("test_user1"),
        }
        .insert_into(app_accounts)
        .get_result(&mut conn)
        .unwrap();

        NewUser { account_id: acc.id }
            .insert_into(app_users::dsl::app_users)
            .execute(&mut conn)
            .unwrap();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .service(web::resource("/profile").route(web::post().to(profile::view_profile))),
        )
        .await;

        let req = test::TestRequest::post().uri("/profile").to_request();

        let res = test::call_service(&app, req).await;

        //Checks
        assert_eq!(res.status(), 200);

        rollback_db(&mut conn).unwrap();
    }
}
