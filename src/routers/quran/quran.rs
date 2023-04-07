use crate::models::{QuranAyah, QuranSurah, QuranWord};
use crate::{error::RouterError, validate::validate, DbPool};
use actix_web::web;
use diesel::prelude::*;
use diesel::{dsl::exists, select};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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

#[derive(Eq, Hash, PartialEq, Serialize, Clone, Debug)]
pub struct SimpleAyah {
    number: i32,
    sajdeh: Option<String>,
}

#[derive(Debug, Serialize, Queryable, Eq, Hash, PartialEq, Clone)]
pub struct SimpleSurah {
    #[serde(skip_serializing)]
    id: i32,
    name: String,
    period: String,
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

pub fn multip<T, U, F, NT>(vector: Vec<(T, U)>, insert_data_type: F) -> HashMap<NT, Vec<U>>
where
    T: Sized + Clone,
    U: Sized,
    NT: Sized + Eq + Hash,
    F: Fn(T) -> NT,
{
    let mut map: HashMap<NT, Vec<U>> = HashMap::new();
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
            return Err(NotFound(format!("Mushaf {} not exists", &query.mushaf)));
        }

        // Validate the mode
        // for now
        if query.mode != Mode::Surah {
            return Err(NotFound(format!("mode {} is not supported", &query.mode)));
        }

        let filter = quran_surahs
            .inner_join(quran_ayahs.inner_join(quran_words))
            .filter(s_id.between(query.from as i32, query.limit.unwrap() as i32));

        let result = filter
            .select((QuranAyah::as_select(), QuranWord::as_select()))
            .load::<(QuranAyah, QuranWord)>(&mut conn)
            .unwrap();

        let res: HashMap<SimpleAyah, Vec<QuranWord>> = multip(result, |ayah| SimpleAyah {
            number: ayah.ayah_number,
            sajdeh: ayah.sajdeh.clone(),
        });

        let mut res = res
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

        res.sort_by(|a, b| a.ayah.number.cmp(&b.ayah.number));

        let surahs = filter
            .select(QuranSurah::as_select())
            .load::<QuranSurah>(&mut conn)
            .unwrap();

        let surahs = surahs
            .into_iter()
            .zip(res.clone())
            .collect::<Vec<(QuranSurah, Ayah)>>();

        let another = multip(surahs, |surah| SimpleSurah {
            id: surah.id,
            name: surah.name,
            period: surah.period,
        });

        let mut res = another
            .into_iter()
            .map(|(surah, ayah_with_words)| QuranResponseData {
                surah,
                ayahs: ayah_with_words,
            })
            .collect::<Vec<QuranResponseData>>();

        res.sort_by(|a, b| a.surah.id.cmp(&b.surah.id));

        Ok(web::Json(res))
    })
    .await
    .unwrap();

    result
}
