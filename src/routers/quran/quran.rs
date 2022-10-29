use crate::{models, validate::validate, DbPool};
use actix_web::{get, web, Error, HttpResponse};
use diesel::prelude::*;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct QuranQuery {
    #[validate(range(min = 1, max = 114))]
    from: i32,

    #[validate(range(min = 1, max = 114))]
    to: i32,
}

#[get("/quran")]
pub async fn quran(
    query: web::Query<QuranQuery>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    use crate::schema::quran_text::dsl::*;

    validate(&query.0)?;

    let result = web::block(move || {
        let mut conn = pool.get().unwrap();

        quran_text
            .filter(surah.between(query.from, query.to))
            .load::<models::QuranText>(&mut conn)
            .unwrap()
    })
    .await?;

    Ok(HttpResponse::Ok().json(result))
}
