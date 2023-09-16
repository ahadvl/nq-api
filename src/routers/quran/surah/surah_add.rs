use std::str::FromStr;

use super::SimpleSurah;
use crate::models::NewQuranSurah;
use crate::{error::RouterError, DbPool};
use actix_web::web;
use diesel::prelude::*;
use uuid::Uuid;

// Add's and new surah
pub async fn surah_add<'a>(
    new_surah: web::Json<SimpleSurah>,
    pool: web::Data<DbPool>,
) -> Result<&'a str, RouterError> {
    use crate::schema::mushafs::dsl::{id as mushaf_id, mushafs, uuid as mushaf_uuid};
    use crate::schema::quran_surahs::dsl::quran_surahs;

    let new_surah = new_surah.into_inner();

    web::block(move || {
        let mut conn = pool.get().unwrap();
        let uuid = Uuid::from_str(&new_surah.mushaf_uuid)?;

        // Select the mushaf by uuid
        // and get the mushaf id
        let mushaf: i32 = mushafs
            .filter(mushaf_uuid.eq(uuid))
            .select(mushaf_id)
            .get_result(&mut conn)?;

        // Add a new surah
        NewQuranSurah {
            name: new_surah.name,
            period: new_surah.period,
            number: new_surah.number,
            mushaf_id: mushaf,
            bismillah_status: new_surah.bismillah_status,
        }
        .insert_into(quran_surahs)
        .execute(&mut conn)?;

        Ok("Added")
    })
    .await
    .unwrap()
}
