use super::SimpleSurah;
use crate::models::NewQuranSurah;
use crate::{error::RouterError, DbPool};
use actix_web::web;
use diesel::prelude::*;

// Add's and new surah
pub async fn surah_add<'a>(
    new_surah: web::Json<SimpleSurah>,
    pool: web::Data<DbPool>,
) -> Result<&'a str, RouterError> {
    use crate::schema::quran_surahs::dsl::quran_surahs;

    let new_surah = new_surah.into_inner();

    web::block(move || {
        let mut conn = pool.get().unwrap();

        // Add a new surah
        NewQuranSurah {
            name: new_surah.name,
            period: new_surah.period,
            number: new_surah.number,
            mushaf_id: new_surah.mushaf_id,
            bismillah_text: new_surah.bismillah_text,
            bismillah_status: new_surah.bismillah_status,
        }
        .insert_into(quran_surahs)
        .execute(&mut conn)?;

        Ok("Added")
    })
    .await
    .unwrap()
}
