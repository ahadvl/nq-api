pub mod send_code;
pub mod verify;

use chrono::{offset::Utc, Duration, NaiveTime};

// This constants will make code length
// equal to 6
pub const MIN_RANDOM_CODE: i32 = 100000;
pub const MAX_RANDOM_CODE: i32 = 999999;

/// Get deference between Current time and past_time
pub fn time_deference(past_time: NaiveTime) -> Duration {
    let current_date = Utc::now().time();

    current_date - past_time
}
