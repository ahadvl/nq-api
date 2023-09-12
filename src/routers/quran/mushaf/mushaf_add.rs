use crate::error::RouterError;
use crate::models::NewQuranMushaf;
use crate::DbPool;
use actix_web::web;
use diesel::prelude::*;

use super::SimpleMushaf;

/// Add's new mushaf
pub async fn mushaf_add<'a>(
    new_mushaf: web::Json<SimpleMushaf>,
    pool: web::Data<DbPool>,
) -> Result<&'a str, RouterError> {
    use crate::schema::mushafs::dsl::mushafs;

    let new_mushaf = new_mushaf.into_inner();

    let result = web::block(move || {
        let mut conn = pool.get().unwrap();

        NewQuranMushaf {
            name: Some(&new_mushaf.name),
            source: Some(&new_mushaf.source),
        }
        .insert_into(mushafs)
        .execute(&mut conn)?;

        Ok("Added")
    })
    .await
    .unwrap();

    result
}
