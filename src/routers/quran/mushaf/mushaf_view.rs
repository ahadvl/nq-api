use std::str::FromStr;

use crate::error::RouterError;
use crate::models::QuranMushaf;
use crate::DbPool;
use ::uuid::Uuid;
use actix_web::web;
use diesel::prelude::*;

/// Return's a single mushaf
pub async fn mushaf_view(
    path: web::Path<String>,
    pool: web::Data<DbPool>,
) -> Result<web::Json<QuranMushaf>, RouterError> {
    use crate::schema::mushafs::dsl::{mushafs, uuid as mushaf_uuid};

    let path = path.into_inner();

    let result = web::block(move || {
        let mut conn = pool.get().unwrap();
        let uuid = Uuid::from_str(&path)?;

        // Get the single mushaf from the database
        let quran_mushafs: QuranMushaf =
            mushafs.filter(mushaf_uuid.eq(uuid)).get_result(&mut conn)?;

        Ok(web::Json(quran_mushafs))
    })
    .await
    .unwrap();

    result
}
