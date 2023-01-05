use chrono::NaiveDate;
use serde::{de, Deserialize, Deserializer};

/// parse the date time
pub fn parse_date_time_with_format<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(de::Error::custom)
}
