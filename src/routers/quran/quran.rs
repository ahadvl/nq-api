use crate::{
    error::RouterError,
    models::{self, QuranText},
    validate::{validate},
    DbPool,
};
use actix_web::web;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Mushaf {
    Hafs,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Surah,
    Ayah,
    Page,
    Juz,
    Hizb,
    Manzil,
    Ruku,
}

#[derive(Deserialize, Validate)]
pub struct QuranQuery {
    #[validate(range(min = 1, max = 114))]
    from: u8,

    #[validate(range(min = 1, max = 114))]
    to: u8,
    
    mushaf: Mushaf,

    mode: Mode,
}

pub async fn quran(
    query: web::Query<QuranQuery>,
    pool: web::Data<DbPool>,
) -> Result<web::Json<Vec<QuranText>>, RouterError> {
    use crate::schema::quran_text::dsl::*;

    validate(&query.0)?;

    let result = web::block(move || {
        let mut conn = pool.get().unwrap();

        // Get quran surahs
        // example BETWEEN 1 to 100
        let Ok(quran) = quran_text
            .filter(surah_id.between(query.from as i32, query.to as i32))
            .load::<models::QuranText>(&mut conn) else {
                return Err(RouterError::InternalError);
            };

        Ok(web::Json(quran))
    })
    .await
    .unwrap();

    result
}
