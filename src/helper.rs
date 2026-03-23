use time::{Date, OffsetDateTime};

pub fn now() -> OffsetDateTime {
    OffsetDateTime::now_local().unwrap() // I think this unwrap is okay
}

pub fn today() -> Date {
    now().date()
}
