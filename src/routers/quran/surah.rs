use crate::models::{QuranAyah, QuranMushaf, QuranSurah, QuranWord};
use crate::routers::multip;
use crate::schema::quran_ayahs::surah_id;
use crate::{error::RouterError, DbPool};
use actix_web::web;
use diesel::dsl::count;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::hash::Hash;
use uuid::Uuid;

/// The query needs the mushaf
/// for example /surah?mushaf=hafs
#[derive(Deserialize)]
pub struct SurahListQuery {
    mushaf: String,
}

/// The response type for /surah/{id}
#[derive(Serialize, Clone, Debug)]
pub struct SingleSurahResponse {
    pub mushaf_uuid: Uuid,
    pub mushaf_name: Option<String>,
    pub surah_uuid: Uuid,
    pub surah_name: String,
    pub surah_period: Option<String>,
    pub surah_number: i32,
    pub bismillah_status: String,
    pub number_of_ayahs: i64,
}

/// The response type for /surah
#[derive(Serialize, Clone, Debug)]
pub struct SurahListResponse {
    pub uuid: Uuid,
    pub name: String,
    pub period: Option<String>,
    pub number: i32,
    pub number_of_ayahs: i64,
}

/// Get the lists of surah
pub async fn surahs_list<'a>(
    query: web::Query<SurahListQuery>,
    pool: web::Data<DbPool>,
) -> Result<web::Json<Vec<SurahListResponse>>, RouterError> {
    use crate::schema::mushafs::dsl::{mushafs, name as mushaf_name};
    use crate::schema::quran_surahs::dsl::*;

    let query = query.into_inner();

    let result = web::block(move || {
        let mut conn = pool.get().unwrap();

        // Select the specific mushaf
        // and check if it exists
        let mushaf = mushafs
            .filter(mushaf_name.eq(&query.mushaf))
            .get_result::<QuranMushaf>(&mut conn)?;

        // Get the list of surahs from the database
        let surahs = quran_surahs
            .filter(mushaf_id.eq(mushaf.id))
            .load::<QuranSurah>(&mut conn)?;

        let ayahs = surahs
            .clone()
            .into_iter()
            .map(|s| {
                QuranAyah::belonging_to(&s)
                    .select(count(surah_id))
                    .get_result(&mut conn)
                    .unwrap()
            })
            .collect::<Vec<i64>>();

        // now iter over the surahs and bind it with
        // number_of_ayahs
        let surahs = surahs
            .into_iter()
            .zip(ayahs)
            .map(|(surah, number_of_ayahs)| SurahListResponse {
                uuid: surah.uuid,
                name: surah.name,
                number: surah.number,
                period: surah.period,
                number_of_ayahs,
            })
            .collect::<Vec<SurahListResponse>>();

        Ok(web::Json(surahs))
    })
    .await
    .unwrap();

    result
}

/// The quran text format
/// Each word has its own uuid
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Format {
    Text,
    Word,
}

impl Default for Format {
    fn default() -> Self {
        Self::Text
    }
}

/// Quran text type
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
enum AyahTextType {
    Words(Vec<QuranWord>),
    Text(String),
}

/// The Ayah type that will return in the response
#[derive(PartialOrd, Ord, Eq, Hash, PartialEq, Serialize, Clone, Debug)]
pub struct SimpleAyah {
    number: i32,
    uuid: Uuid,
    sajdeh: Option<String>,
}

/// it contains ayah info and the content
#[derive(Serialize, Clone, Debug)]
pub struct Ayah {
    #[serde(flatten)]
    ayah: SimpleAyah,
    content: AyahTextType,
}

/// The final response body
#[derive(Serialize, Clone, Debug)]
pub struct QuranResponseData {
    #[serde(flatten)]
    surah: SingleSurahResponse,
    ayahs: Vec<Ayah>,
}

/// the query for the /surah/{uuid}
/// example /surah/{uuid}?format=word
#[derive(Debug, Clone, Deserialize)]
pub struct GetSurahQuery {
    #[serde(default)]
    format: Format,
}

pub async fn surah(
    path: web::Path<String>,
    query: web::Query<GetSurahQuery>,
    pool: web::Data<DbPool>,
) -> Result<web::Json<QuranResponseData>, RouterError> {
    use crate::schema::mushafs::dsl::{id as mushaf_id, mushafs};
    use crate::schema::quran_ayahs::dsl::quran_ayahs;
    use crate::schema::quran_surahs::dsl::quran_surahs;
    use crate::schema::quran_surahs::dsl::uuid as surah_uuid;
    use crate::schema::quran_words::dsl::quran_words;

    let query = query.into_inner();
    let path = path.into_inner();

    let result = web::block(move || {
        let mut conn = pool.get().unwrap();

        let uuid = Uuid::parse_str(&path)?;

        let result = quran_surahs
            .filter(surah_uuid.eq(uuid))
            .inner_join(quran_ayahs.inner_join(quran_words))
            .select((QuranAyah::as_select(), QuranWord::as_select()))
            .load::<(QuranAyah, QuranWord)>(&mut conn)?;

        let ayahs_as_map: BTreeMap<SimpleAyah, Vec<QuranWord>> =
            multip(result, |ayah| SimpleAyah {
                number: ayah.ayah_number,
                uuid: ayah.uuid,
                sajdeh: ayah.sajdeh,
            });

        let final_ayahs = ayahs_as_map
            .into_iter()
            .map(|(ayah, words)| Ayah {
                ayah,
                content: match query.format {
                    Format::Text => AyahTextType::Text(
                        words
                            .into_iter()
                            .map(|qword| qword.word)
                            .collect::<Vec<String>>()
                            .join(" "),
                    ),
                    Format::Word => AyahTextType::Words(words),
                },
            })
            .collect::<Vec<Ayah>>();

        // Get the surah
        let surah = quran_surahs
            .filter(surah_uuid.eq(uuid))
            .get_result::<QuranSurah>(&mut conn)?;

        // Get the mushaf
        let mushaf = mushafs
            .filter(mushaf_id.eq(surah.mushaf_id))
            .get_result::<QuranMushaf>(&mut conn)?;

        Ok(web::Json(QuranResponseData {
            surah: SingleSurahResponse {
                mushaf_uuid: mushaf.uuid,
                mushaf_name: mushaf.name,
                surah_uuid: surah.uuid,
                surah_name: surah.name,
                surah_period: surah.period,
                surah_number: surah.number,
                bismillah_status: surah.bismillah_status,
                number_of_ayahs: final_ayahs.len() as i64,
            },
            ayahs: final_ayahs,
        }))
    })
    .await
    .unwrap();

    result
}
