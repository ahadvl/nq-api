use crate::models::{QuranAyah, QuranSurah, QuranWord};
use crate::{error::RouterError, validate::validate, DbPool};
use actix_web::web;
use diesel::prelude::*;
use diesel::{dsl::exists, select};
use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};
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

#[derive(Debug, Serialize, Queryable, Clone)]
pub struct ViewableWord {
    id: i32,
    ayah_id: i32,
    word: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
enum AyahTextType {
    Words(Vec<ViewableWord>),
    Text(String),
}

#[derive(Debug)]
pub struct ViewableAyah {
    number: i32,
    sajdeh: Option<String>,
    content: AyahTextType,
}

impl Serialize for ViewableAyah {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ViewableAyah", 3)?;
        state.serialize_field("number", &self.number)?;

        let content_name = match &self.content {
            AyahTextType::Text(_) => "text",
            AyahTextType::Words(_) => "words",
        };

        state.serialize_field("sajdeh", &self.sajdeh)?;
        state.serialize_field(content_name, &self.content)?;
        state.end()
    }
}

#[derive(Eq, Hash, PartialEq, Serialize, Clone, Debug)]
pub struct SimpleAyah {
    ayah_id: i32,
    sajdeh: Option<String>,
}

#[derive(Debug, Serialize, Queryable, Eq, Hash, PartialEq, Clone)]
pub struct SimpleSurah {
    surah_id: i32,
    surah_name: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct Res {
    #[serde(flatten)]
    ayah: SimpleAyah,
    words: Vec<QuranWord>,
}

#[derive(Serialize, Clone, Debug)]
pub struct FinalRes {
    #[serde(flatten)]
    surah: SimpleSurah,
    ayahs: Vec<Res>,
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
) -> Result<web::Json<Vec<FinalRes>>, RouterError> {
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

        // println!("{:?}", n_ayahs);

        let filter = quran_surahs
            .inner_join(quran_ayahs.inner_join(quran_words))
            .filter(s_id.between(query.from as i32, query.limit.unwrap() as i32));

        let result = filter
            .select((QuranAyah::as_select(), QuranWord::as_select()))
            .load::<(QuranAyah, QuranWord)>(&mut conn)
            .unwrap();

        let res: HashMap<SimpleAyah, Vec<QuranWord>> = multip(result, |ayah| SimpleAyah {
            ayah_id: ayah.id,
            sajdeh: ayah.sajdeh.clone(),
        });

        let mut res = res
            .into_iter()
            .map(|(ayah, words)| Res { ayah, words })
            .collect::<Vec<Res>>();

        res.sort_by(|a, b| a.ayah.ayah_id.cmp(&b.ayah.ayah_id));

        let surahs = filter
            .select(QuranSurah::as_select())
            .load::<QuranSurah>(&mut conn)
            .unwrap();

        let surahs = surahs
            .into_iter()
            .zip(res.clone())
            .collect::<Vec<(QuranSurah, Res)>>();

        let another = multip(surahs, |surah| SimpleSurah {
            surah_id: surah.id,
            surah_name: surah.name,
        });

        let mut res = another
            .into_iter()
            .map(|(surah, ayah_with_words)| FinalRes {
                surah,
                ayahs: ayah_with_words,
            })
            .collect::<Vec<FinalRes>>();

        res.sort_by(|a, b| a.surah.surah_id.cmp(&b.surah.surah_id));

        Ok(web::Json(res))
    })
    .await
    .unwrap();

    result
}
