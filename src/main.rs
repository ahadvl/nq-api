use actix_web::{web, App, HttpServer};

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use std::env;

mod models;
mod routers;
mod schema;

type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> ConnectionManager<PgConnection> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    ConnectionManager::<PgConnection>::new(database_url)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pg_manager = establish_connection();

    let pool = Pool::builder()
        .build(pg_manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(routers::account::send_code)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
