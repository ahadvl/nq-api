use std::str::FromStr;

use crate::error::RouterError;
use crate::DbPool;
use actix_web::web;
use diesel::prelude::*;
use uuid::Uuid;

use super::SimpleAyah;

/// Update's single ayah
pub async fn ayah_edit<'a>(
    path: web::Path<String>,
    new_ayah: web::Json<SimpleAyah>,
    pool: web::Data<DbPool>,
    data: web::ReqData<u32>,
) -> Result<&'a str, RouterError> {
    use crate::schema::app_users::dsl::{account_id as user_acc_id, app_users, id as user_id};
    use crate::schema::quran_ayahs::dsl::{
        ayah_number, creator_user_id, quran_ayahs, sajdeh as ayah_sajdeh,
        surah_id as ayah_surah_id, uuid as ayah_uuid,
    };
    use crate::schema::quran_surahs::dsl::{id as surah_id, quran_surahs, uuid as surah_uuid};

    let new_ayah = new_ayah.into_inner();
    let path = path.into_inner();
    let data = data.into_inner();

    let result = web::block(move || {
        let mut conn = pool.get().unwrap();
        let new_ayah_uuid = Uuid::from_str(&path)?;

        // Get the creator user-id
        let user: i32 = app_users
            .filter(user_acc_id.eq(data as i32))
            .select(user_id)
            .get_result(&mut conn)?;

        // Get the target surah by surah-uuid
        let target_surah: i32 = quran_surahs
            .filter(surah_uuid.eq(Uuid::from_str(&new_ayah.surah_uuid)?))
            .select(surah_id)
            .get_result(&mut conn)?;

        diesel::update(quran_ayahs.filter(ayah_uuid.eq(new_ayah_uuid)))
            .set((
                ayah_number.eq(new_ayah.ayah_number),
                ayah_surah_id.eq(target_surah),
                creator_user_id.eq(user),
                ayah_sajdeh.eq(new_ayah.sajdeh)
            ))
            .execute(&mut conn)?;

        Ok("Edited")
    })
    .await
    .unwrap();

    result
}
