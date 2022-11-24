use actix_cors::Cors;
use actix_web::{web, App, HttpServer};

use auth::token::TokenAuth;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use email::EmailManager;
use lettre::transport::smtp::authentication::Credentials;
use std::env;
use token_checker::TokenFromDatabase;

mod email;
mod models;
mod routers;
mod schema;
mod test;
mod token_checker;
mod validate;

use routers::account::send_code;
use routers::account::verify;
use routers::profile::profile;
use routers::quran::quran;

type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn create_emailer() -> EmailManager {
    dotenv().ok();

    let host = env::var("SMTP_HOST").expect("SMTP_HOST must be set");
    let port = env::var("SMTP_PORT").expect("SMTP_PORT must be set");
    let from = env::var("SMTP_FROM").expect("SMTP_FROM must be set");
    let username = env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set");
    let password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set");

    let credentials = Credentials::new(username, password);

    EmailManager::new(&host, port.parse().unwrap(), credentials, from)
        .expect("Cant create EmailManager")
}

pub fn establish_database_connection() -> ConnectionManager<PgConnection> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    ConnectionManager::<PgConnection>::new(database_url)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pg_manager = establish_database_connection();

    let pool = Pool::builder()
        .build(pg_manager)
        .expect("Failed to create pool.");

    let mailer = create_emailer();

    let token_checker = TokenFromDatabase::new(pool.clone());

    HttpServer::new(move || {
        // Set All to the cors
        let cors = Cors::default().supports_credentials();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(mailer.clone()))
            .service(send_code::send_code)
            .service(verify::verify)
            .service(quran::quran)
            .service(
                web::resource("/profile")
                    .wrap(TokenAuth::new(token_checker.clone()))
                    .route(web::get().to(profile::view_profile)),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
