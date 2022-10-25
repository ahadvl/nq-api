pub mod send_code;
pub mod verify;

use chrono::{offset::Utc, Duration, NaiveTime};

/// Get deference between Current time and past_time
pub fn time_deference(past_time: NaiveTime) -> Duration {
    let current_date = Utc::now().time();
    let diff = current_date - past_time;

    diff
}
