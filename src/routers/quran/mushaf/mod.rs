pub mod mushaf_list;
pub mod mushaf_add;
pub mod mushaf_view;
pub mod mushaf_edit;
pub mod mushaf_delete;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct SimpleMushaf {
    name: String,
    source: String,
}

