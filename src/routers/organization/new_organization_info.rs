use crate::datetime::parse_date_time_with_format;
use chrono::NaiveDate;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct NewOrgInfo {
    pub username: String,
    pub name: String,
    pub profile_image: Option<String>,

    #[serde(deserialize_with = "parse_date_time_with_format")]
    pub established_date: NaiveDate,

    #[validate(length(equal = 11))]
    pub national_id: String,
}
