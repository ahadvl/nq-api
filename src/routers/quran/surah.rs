use crate::error::RouterError;
use crate::models::QuranSurah;
use crate::DbPool;
use actix_web::web;
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SurahListQuery {
    mushaf: String,
}

/// Get the lists of surah
pub async fn surah(
    query: web::Query<SurahListQuery>,
    pool: web::Data<DbPool>,
) -> Result<web::Json<Vec<QuranSurah>>, RouterError> {
    use crate::error::RouterError::*;
    use crate::schema::quran_surahs::dsl::*;

    let query = query.into_inner();

    // TODO: fix
    if query.mushaf != "hafs".to_string() {
        return Err(NotFound(format!(
            "Mushaf {} is not supported for now",
            query.mushaf
        )));
    }

    let result = web::block(move || {
        let Ok(mut conn )= pool.get() else {
            return Err(InternalError);
        };

        // Get the list of surahs from the database
        let Ok(surahs) = quran_surahs.load::<QuranSurah>(&mut conn) else {
            return Err(InternalError);
        };

        Ok(web::Json(surahs))
    })
    .await
    .unwrap();

    result
}
