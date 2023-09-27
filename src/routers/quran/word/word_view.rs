use std::str::FromStr;

use crate::error::RouterError;
use crate::models::QuranWord;
use crate::DbPool;
use ::uuid::Uuid;
use actix_web::web;
use diesel::prelude::*;

/// Return's a single word
pub async fn word_view(
    path: web::Path<String>,
    pool: web::Data<DbPool>,
) -> Result<web::Json<QuranWord>, RouterError> {
    use crate::schema::quran_words::dsl::{quran_words, uuid as word_uuid};

    let path = path.into_inner();

    let result = web::block(move || {
        let mut conn = pool.get().unwrap();
        let uuid = Uuid::from_str(&path)?;

        // Get the single word from the database
        let quran_word: QuranWord = quran_words
            .filter(word_uuid.eq(uuid))
            .get_result(&mut conn)?;

        Ok(web::Json(quran_word))
    })
    .await
    .unwrap();

    result
}
