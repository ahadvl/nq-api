use crate::{models::Organization, DbPool};
use actix_web::{web, Responder};
use diesel::prelude::*;

pub async fn get_list_of_organizations(pool: web::Data<DbPool>) -> impl Responder {
    use crate::schema::app_organizations_table::dsl::*;

    let organizations = web::block(move || {
        let mut conn = pool.get().unwrap();

        let select_all = app_organizations_table
            .load::<Organization>(&mut conn)
            .unwrap();

        return select_all;
    })
    .await
    .unwrap();

    web::Json(organizations)
}
