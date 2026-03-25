use time::{Date, OffsetDateTime};

pub fn now() -> anyhow::Result<OffsetDateTime> {
    Ok(OffsetDateTime::now_local()?)
}

pub fn today() -> anyhow::Result<Date> {
    Ok(now()?.date())
}
