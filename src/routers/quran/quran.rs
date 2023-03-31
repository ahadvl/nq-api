use crate::models::{QuranAyah, QuranSurah, QuranWord};
use crate::{error::RouterError, validate::validate, DbPool};
use actix_web::web;
use diesel::prelude::*;
use diesel::{dsl::exists, select};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
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

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Format {
    Ayah,
    Word,
}

#[derive(Clone, Deserialize, Validate)]
pub struct QuranQuery {
    #[validate(range(min = 1, max = 114))]
    from: u8,

    #[validate(range(min = 1, max = 114))]
    limit: u8,

    mode: Mode,

    mushaf: String,

    format: Format,
}

#[derive(Debug, Serialize)]
pub struct ViewablWord {
    word: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
enum AyahTextType {
    Words(Vec<QuranWord>),
    Text(String),
}

#[derive(Debug, Serialize)]
pub struct ViewableAyah {
    number: i32,
    sajdeh: Option<String>,
    text: AyahTextType,
}

#[derive(Debug, Serialize)]
pub struct SurahWithAyahs {
    #[serde(flatten)]
    surah: QuranSurah,
    ayahs: Vec<ViewableAyah>,
}

pub async fn quran(
    query: web::Query<QuranQuery>,
    pool: web::Data<DbPool>,
) -> Result<web::Json<Vec<SurahWithAyahs>>, RouterError> {
    use crate::error::RouterError::*;
    use crate::schema::mushafs;
    use crate::schema::quran_surahs::dsl::{id as s_id, quran_surahs};

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
            return Err(NotFound(format!("mode {} not exists", &query.mode)));
        }

        let Ok(surahs) = quran_surahs.filter(s_id.between(query.from as i32, query.limit as i32))
            .get_results::<QuranSurah>(&mut conn) else {
                return Err(InternalError)
            };

        let Ok(ayahs) = QuranAyah::belonging_to(&surahs)
            .get_results::<QuranAyah>(&mut conn) else {
                return Err(InternalError)
            };

        let Ok(content) = QuranWord::belonging_to(&ayahs)
            // .select(QuranWord::as_select())
            .get_results::<QuranWord>(&mut conn) else {
                return Err(InternalError);
            };

        let result = ayahs
            .clone()
            .grouped_by(&surahs)
            .into_iter()
            .zip(surahs.clone())
            .map(|(ayahs, surah)| SurahWithAyahs {
                surah,
                ayahs: ayahs
                    .into_iter()
                    .map(|ayah| ViewableAyah {
                        number: ayah.ayah_number,
                        sajdeh: ayah.sajdeh,
                        text: match query.format {
                            Format::Word => AyahTextType::Words(
                                content
                                    .clone()
                                    .into_iter()
                                    .filter(|w| w.ayah_id == ayah.id)
                                    .collect::<Vec<QuranWord>>(),
                            ),

                            Format::Ayah => AyahTextType::Text(
                                content
                                    .clone()
                                    .into_iter()
                                    .filter(|w| w.ayah_id == ayah.id)
                                    .map(|w| w.word)
                                    .collect::<Vec<String>>()
                                    .join(" "),
                            ),
                        },
                    })
                    .collect::<Vec<ViewableAyah>>(),
            })
            .collect::<Vec<SurahWithAyahs>>();

        Ok(web::Json(result))
    })
    .await
    .unwrap();

    result
}
