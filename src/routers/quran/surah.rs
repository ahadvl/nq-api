use crate::models::{QuranAyah, QuranSurah, QuranWord};
use crate::{error::RouterError, DbPool};
use actix_web::web;
use diesel::prelude::*;
use diesel::{dsl::exists, select};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::hash::Hash;

// TODO: maybe change the localtion of this function ?
// TODO: write documentation for this function
// TODO: find the better name
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

/// Get the lists of surah
pub async fn surahs_list(
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
    id: i32,
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
    surah: QuranSurah,
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
        let Ok(mut conn )= pool.get() else {
            return Err(InternalError);
        };

        // Select the specific mushaf
        // and check if it exists
        let Ok(exists)= select(exists(
            mushafs.filter(mushaf_name.eq(&query.mushaf)),
        ))
        .get_result::<bool>(&mut conn)
        else {
            return Err(InternalError)
        };

        if exists == false {
            return Err(NotFound(format!(
                "Mushaf {} is not supported for now",
                &query.mushaf
            )));
        }

        let Ok(result) = quran_surahs
            .filter(surah_number.eq(path as i32))
            .inner_join(quran_ayahs.inner_join(quran_words))
            .select((QuranAyah::as_select(), QuranWord::as_select()))
            .load::<(QuranAyah, QuranWord)>(&mut conn) else {
                return Err(InternalError);
            };

        let ayahs_as_map: BTreeMap<SimpleAyah, Vec<QuranWord>> =
            multip(result, |ayah| SimpleAyah {
                id: ayah.id,
                number: ayah.ayah_number,
                sajdeh: ayah.sajdeh.clone(),
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
            surah,
            ayahs: final_ayahs,
        }))
    })
    .await
    .unwrap();

    result
}
