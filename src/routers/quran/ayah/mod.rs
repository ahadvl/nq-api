pub mod ayah_list;
pub mod ayah_view;
pub mod ayah_delete;
pub mod ayah_edit;
pub mod ayah_add;

use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "lowercase")] 
pub enum SajdehType {
    Mostahab,
    Vajib,
}

#[derive(Serialize, Deserialize)]
pub struct SimpleAyah {
    pub surah_uuid: String,
    pub ayah_number: i32,
    pub sajdeh: Option<SajdehType>,
}
