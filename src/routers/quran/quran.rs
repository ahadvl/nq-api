use crate::models::{QuranAyah, QuranSurah, QuranWord};
use crate::{error::RouterError, validate::validate, DbPool};
use actix_web::web;
use diesel::prelude::*;
use diesel::{dsl::exists, select};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::Display;
use std::hash::Hash;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Surah,
    Ayah,
    Page,
    Juz,
    Hizb,
    Manzil,
    Ruku,
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Ayah => write!(f, "ayah"),
            Mode::Surah => write!(f, "surah"),
            Mode::Page => write!(f, "page"),
            Mode::Juz => write!(f, "juz"),
            Mode::Hizb => write!(f, "hizb"),
            Mode::Manzil => write!(f, "manzil"),
            Mode::Ruku => write!(f, "ruku"),
        }
    }
}

impl Default for Mode {
    fn default() -> Self {
        Self::Surah
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Format {
    Ayah,
    Word,
}

impl Default for Format {
    fn default() -> Self {
        Self::Ayah
    }
}

#[derive(Clone, Deserialize, Validate)]
pub struct QuranQuery {
    #[validate(range(min = 1, max = 114))]
    from: u32,

    #[validate(range(min = 0, max = 114))]
    limit: Option<u32>,

    #[serde(default)]
    mode: Mode,

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

#[derive(Debug, Serialize, Queryable, Eq, Hash, PartialEq, Clone, PartialOrd, Ord)]
pub struct SimpleSurah {
    id: i32,
    name: String,
    period: Option<String>,
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

pub async fn quran(
    query: web::Query<QuranQuery>,
    pool: web::Data<DbPool>,
) -> Result<web::Json<Vec<QuranResponseData>>, RouterError> {
    use crate::error::RouterError::*;
    use crate::schema::mushafs;
    use crate::schema::quran_surahs::dsl::{id as s_id, quran_surahs};
    use crate::schema::{quran_ayahs::dsl::quran_ayahs, quran_words::dsl::quran_words};

    validate(&query.0)?;

    let result = web::block(move || {
        let mut conn = pool.get().unwrap();

        // Select the specific mushaf
        // and check if it exists
        let Ok(exists)= select(exists(
            mushafs::dsl::mushafs.filter(mushafs::dsl::name.eq(&query.mushaf)),
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

        // Validate the mode
        // for now
        if query.mode != Mode::Surah {
            return Err(NotFound(format!("mode {} is not supported", &query.mode)));
        }

        let Ok(result) = quran_surahs
            .filter(s_id.between(query.from as i32, query.limit.unwrap() as i32))
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
                    Format::Ayah => AyahTextType::Text(
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

        let Ok(surahs) = quran_surahs
            .filter(s_id.between(query.from as i32, query.limit.unwrap() as i32))
            .inner_join(quran_ayahs)
            .select((QuranSurah::as_select(), QuranAyah::as_select()))
            .load::<(QuranSurah, QuranAyah)>(&mut conn) else {
                return Err(InternalError);
            };

        let surahs = surahs
            .into_iter()
            .zip(final_ayahs.clone())
            // FIX: Maybe better way?
            .map(|((surah, _), ayah)| (surah, ayah))
            .collect::<Vec<(QuranSurah, Ayah)>>();

        let surahs_as_map = multip(surahs, |surah| SimpleSurah {
            id: surah.id,
            name: surah.name,
            period: surah.period,
        });

        let final_response = surahs_as_map
            .into_iter()
            .map(|(surah, ayah_with_words)| QuranResponseData {
                surah,
                ayahs: ayah_with_words,
            })
            .collect::<Vec<QuranResponseData>>();

        Ok(web::Json(final_response))
    })
    .await
    .unwrap();

    result
}
