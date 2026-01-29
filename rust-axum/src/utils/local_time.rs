use std::time::Duration;

use chrono::{NaiveDateTime, Utc};
use time::OffsetDateTime;

/// Get time for entities
pub fn chrono_time_now() -> NaiveDateTime {
    Utc::now().naive_utc()
}

/// Get time for cookies
pub fn cookie_time_expires() -> OffsetDateTime {
    OffsetDateTime::now_utc() + Duration::from_hours(12)
}
