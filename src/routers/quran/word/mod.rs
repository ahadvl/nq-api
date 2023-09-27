pub mod word_list;
pub mod word_view;
pub mod word_edit;
pub mod word_delete;

use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct SimpleWord {
    pub ayah_uuid: Uuid,
    pub word: String,
}
