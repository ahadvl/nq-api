use crate::{models, query_validator::*, DbPool};
use actix_web::{get, web, Error, HttpResponse};
use diesel::prelude::*;
use serde::Deserialize;

const QURAN_TOTAL_SURAH: i32 = 114;

#[derive(Deserialize)]
pub struct QuranQuery {
    from: i32,
    to: i32,
}

impl QueryValidator for QuranQuery {
    fn validate(&self) -> Result<(), QueryError> {
        if self.from <= 0 {
            return Err(QueryError::new(
                QueryErrorKind::Length,
                "From param must not be smaller than 0".to_string(),
            ));
        } else if self.to > QURAN_TOTAL_SURAH {
            return Err(QueryError::new(
                QueryErrorKind::Length,
                "To param is more than quran total surah: to > 114".to_string(),
            ));
        }

        Ok(())
    }
}

#[get("/quran")]
pub async fn quran(
    query: web::Query<QuranQuery>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    use crate::schema::quran_text::dsl::*;

    let result = web::block(move || {
        query.validate()?;

        let mut conn = pool.get().unwrap();

        quran_text
            .filter(surah.between(query.from, query.to))
            .load::<models::QuranText>(&mut conn)
    })
    .await?;

    Ok(HttpResponse::Ok().json(result))
}
