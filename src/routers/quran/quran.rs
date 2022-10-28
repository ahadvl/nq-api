use crate::{models, DbPool};
use actix_web::{get, web, Error, HttpResponse};
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct QuranQuery {
    from: i32,
    to: i32,
}

#[get("/quran")]
pub async fn quran(
    query: web::Query<QuranQuery>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    use crate::schema::quran_text::dsl::*;

    let result = web::block(move || {
        let mut conn = pool.get().unwrap();

        quran_text
            .filter(surah.between(query.from, query.to))
            .load::<models::QuranText>(&mut conn)
            .expect("Cant get quran_text")
    })
    .await?;

    Ok(HttpResponse::Ok().json(result))
}
