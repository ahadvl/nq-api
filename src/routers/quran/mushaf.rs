use crate::error::RouterError;
use crate::models::QuranMushaf;
use crate::DbPool;
use actix_web::web;
use diesel::prelude::*;

/// Get the lists of mushafs
pub async fn mushaf(pool: web::Data<DbPool>) -> Result<web::Json<Vec<QuranMushaf>>, RouterError> {
    use crate::error::RouterError::*;
    use crate::schema::mushafs::dsl::*;

    let result = web::block(move || {
        let Ok(mut conn )= pool.get() else {
            return Err(InternalError);
        };

        // Get the list of mushafs from the database
        let Ok(quran_mushafs) = mushafs.load::<QuranMushaf>(&mut conn) else {
            return Err(InternalError);
        };

        Ok(web::Json(quran_mushafs))
    })
    .await
    .unwrap();

    result
}
