pub mod ayah_list;
pub mod ayah_view;
pub mod ayah_delete;
pub mod ayah_edit;
pub mod ayah_add;

use std::fmt::Display;

use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "lowercase")] 
pub enum SajdehType {
    Mostahab,
    Vajib,
}

impl Display for SajdehType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mostahab => write!(f, "mostahab"),
            Self::Vajib => write!(f, "vajib"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SimpleAyah {
    pub surah_uuid: String,
    pub ayah_number: i32,
    pub sajdeh: Option<SajdehType>,
}
