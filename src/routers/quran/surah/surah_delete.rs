use crate::{error::RouterError, DbPool};
use actix_web::web;
use uuid::Uuid;

/// Delete's the specific surah
pub async fn surah_delete<'a>(
    path: web::Path<String>,
    pool: web::Data<DbPool>,
) -> Result<&'a str, RouterError> {
    let path = path.into_inner();

    web::block(move || {
        let mut conn = pool.get().unwrap();
        let uuid = Uuid::from_str();

        // Remove the Surah by uuid
        Ok("Deleted")
    }).await.unwrap()
}
