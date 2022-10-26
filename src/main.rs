use actix_web::{web, App, HttpServer};

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use email::EmailManager;
use lettre::transport::smtp::authentication::Credentials;
use std::env;

mod email;
mod models;
mod routers;
mod schema;

use routers::account::send_code;
use routers::account::verify;

type DbPool = Pool<ConnectionManager<PgConnection>>;

fn create_emailer() -> EmailManager {
    dotenv().ok();

    let host = env::var("SMTP_HOST").expect("SMTP_HOST must be set");
    let port = env::var("SMTP_PORT").expect("SMTP_PORT must be set");
    let username = env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set");
    let password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set");

    let credentials = Credentials::new(username, password);

    EmailManager::new(
        &host,
        port.parse().unwrap(),
        credentials,
        "telifesite@gmail.com".to_string(),
    )
    .expect("Cant create EmailManager")
}

fn establish_connection() -> ConnectionManager<PgConnection> {
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

    let mailer = create_emailer();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(mailer.clone()))
            .service(send_code::send_code)
            .service(verify::verify)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
