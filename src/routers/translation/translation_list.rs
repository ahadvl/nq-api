use crate::error::RouterError;
use crate::models::Translation;
use crate::DbPool;
use actix_web::web;
use diesel::prelude::*;

/// Returns the list of translations
pub async fn translation_list(
    pool: web::Data<DbPool>,
) -> Result<web::Json<Vec<Translation>>, RouterError> {
    use crate::schema::translations::dsl::*;

    let result = web::block(move || {
        let mut conn = pool.get().unwrap();

        // Get the list of translations from the database
        let translations_list = translations.load::<Translation>(&mut conn)?;

        Ok(web::Json(translations_list))
    })
    .await
    .unwrap();

    result
}
