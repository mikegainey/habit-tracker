use time::{Date, OffsetDateTime};

pub fn now() -> OffsetDateTime {
    OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc())
}

pub fn today() -> Date {
    now().date()
}
