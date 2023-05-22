use crate::models::{QuranAyah, QuranMushaf, QuranSurah, QuranWord};
use crate::schema::quran_ayahs::surah_id;
use crate::schema::quran_surahs::mushaf_id;
use crate::{error::RouterError, DbPool};
use actix_web::web;
use diesel::dsl::count;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::hash::Hash;
use uuid::Uuid;

/// finds the relatives in the vector
/// Vec<(Obj1, Obj2)>
/// This will collect the Obj2 that related to the Obj1 and returns
/// a BTreeMap (We want the elements be in order)
pub fn multip<T, U, F, NT>(vector: Vec<(T, U)>, insert_data_type: F) -> BTreeMap<NT, Vec<U>>
where
    T: Sized + Clone,
    U: Sized,
    NT: Sized + Eq + Hash + Ord,
    F: Fn(T) -> NT,
{
    let mut map: BTreeMap<NT, Vec<U>> = BTreeMap::new();
    for item in vector {
        if let Some(w) = map.get_mut(&insert_data_type(item.0.clone())) {
            w.push(item.1)
        } else {
            map.insert(insert_data_type(item.0), vec![item.1]);
        }
    }

    map
}

#[derive(Deserialize)]
pub struct SurahListQuery {
    mushaf: String,
}

#[derive(Serialize, Queryable, Clone, Debug)]
pub struct SimpleSurah {
    pub uuid: Uuid,
    pub name: String,
    pub period: Option<String>,
    pub number: i32,
    pub number_of_ayahs: i64,
}

/// Get the lists of surah
pub async fn surahs_list(
    query: web::Query<SurahListQuery>,
    pool: web::Data<DbPool>,
) -> Result<web::Json<Vec<SimpleSurah>>, RouterError> {
    use crate::error::RouterError::*;
    use crate::schema::mushafs::dsl::{mushafs, name as mushaf_name};
    use crate::schema::quran_surahs::dsl::*;

    let query = query.into_inner();

    let result = web::block(move || {
        let Ok(mut conn)= pool.get() else {
            return Err(InternalError);
        };

        // Select the specific mushaf
        // and check if it exists
        let Ok(mushaf) =
            mushafs.filter(mushaf_name.eq(&query.mushaf))
            .get_result::<QuranMushaf>(&mut conn)
        else {
            return Err(NotFound("Mushaf is not supported yet!".to_string()));
        };

        // Get the list of surahs from the database
        let Ok(surahs) = quran_surahs
            .filter(mushaf_id.eq(mushaf.id))
            .load::<QuranSurah>(&mut conn) else {
               return Err(InternalError)
            };

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
            .map(|(surah, number_of_ayahs)| SimpleSurah {
                uuid: surah.uuid,
                name: surah.name,
                number: surah.number,
                period: surah.period,
                number_of_ayahs,
            })
            .collect::<Vec<SimpleSurah>>();

        Ok(web::Json(surahs))
    })
    .await
    .unwrap();

    result
}

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

#[derive(Clone, Deserialize)]
pub struct QuranQuery {
    #[serde(default)]
    format: Format,

    mushaf: String,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
enum AyahTextType {
    Words(Vec<QuranWord>),
    Text(String),
}

#[derive(PartialOrd, Ord, Eq, Hash, PartialEq, Serialize, Clone, Debug)]
pub struct SimpleAyah {
    uuid: Uuid,
    number: i32,
    sajdeh: Option<String>,
}

#[derive(Serialize, Clone, Debug)]
pub struct Ayah {
    #[serde(flatten)]
    ayah: SimpleAyah,
    content: AyahTextType,
}

#[derive(Serialize, Clone, Debug)]
pub struct QuranResponseData {
    #[serde(flatten)]
    surah: SimpleSurah,
    ayahs: Vec<Ayah>,
}

pub async fn surah(
    path: web::Path<u32>,
    query: web::Query<QuranQuery>,
    pool: web::Data<DbPool>,
) -> Result<web::Json<QuranResponseData>, RouterError> {
    use crate::error::RouterError::*;
    use crate::schema::mushafs::dsl::{mushafs, name as mushaf_name};
    use crate::schema::quran_ayahs::dsl::quran_ayahs;
    use crate::schema::quran_surahs::dsl::number as surah_number;
    use crate::schema::quran_surahs::dsl::quran_surahs;
    use crate::schema::quran_words::dsl::quran_words;

    let query = query.into_inner();
    let path = path.into_inner();

    let result = web::block(move || {
        let Ok(mut conn)= pool.get() else {
            return Err(InternalError);
        };

        // Select the specific mushaf
        // and check if it exists
        let Ok(mushaf)=
            mushafs.filter(mushaf_name.eq(&query.mushaf))
            .get_result::<QuranMushaf>(&mut conn)
        else {
            return Err(NotFound("Mushaf is not supported yet!".to_string()));
        };

        let Ok(result) = quran_surahs
            .filter(surah_number.eq(path as i32))
            .filter(mushaf_id.eq(mushaf.id))
            .inner_join(quran_ayahs.inner_join(quran_words))
            .select((QuranAyah::as_select(), QuranWord::as_select()))
            .load::<(QuranAyah, QuranWord)>(&mut conn) else {
                return Err(InternalError);
            };

        let ayahs_as_map: BTreeMap<SimpleAyah, Vec<QuranWord>> =
            multip(result, |ayah| SimpleAyah {
                uuid: ayah.uuid,
                number: ayah.ayah_number,
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
        let Ok(surah)= quran_surahs
            .filter(surah_number.eq(path as i32))
            .get_result::<QuranSurah>(&mut conn) else {
                return Err(InternalError);
            };

        Ok(web::Json(QuranResponseData {
            surah: SimpleSurah {
                uuid: surah.uuid,
                name: surah.name,
                period: surah.period,
                number: surah.number,
                number_of_ayahs: final_ayahs.len() as i64,
            },
            ayahs: final_ayahs,
        }))
    })
    .await
    .unwrap();

    result
}
