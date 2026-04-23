use serde::{Deserialize, Deserializer, Serialize, Serializer};
use time::{OffsetDateTime, format_description::FormatItem, macros::format_description};

static TIMESTAMP_FORMAT: &[FormatItem<'static>] = format_description!(
    "[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour sign:mandatory]:[offset_minute]"
);

pub fn serialize<S>(vec: &Vec<OffsetDateTime>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let strings: Vec<String> = vec
        .iter()
        .map(|dt| {
            dt.format(TIMESTAMP_FORMAT)
                .map_err(serde::ser::Error::custom)
        })
        .collect::<Result<_, _>>()?;

    strings.serialize(serializer)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<OffsetDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let strings = Vec::<String>::deserialize(deserializer)?;

    strings
        .into_iter()
        .map(|s| OffsetDateTime::parse(&s, TIMESTAMP_FORMAT).map_err(serde::de::Error::custom))
        .collect()
}
