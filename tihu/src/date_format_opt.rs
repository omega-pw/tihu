use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Deserializer, Serializer};

pub const FORMAT: &'static str = "%Y-%m-%d %z";

// The signature of a serialize_with function must follow the pattern:
//
//    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
//    where
//        S: Serializer
//
// although it may also be generic over the input types T.
pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match date {
        Some(ref val) => {
            let s = format!("{}", val.format(FORMAT));
            serializer.serialize_str(&s)
        }
        None => serializer.serialize_none(),
    }
}

// The signature of a deserialize_with function must follow the pattern:
//
//    fn deserialize<'de, D>(D) -> Result<T, D::Error>
//    where
//        D: Deserializer<'de>
//
// although it may also be generic over the output types T.
pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = Option::<String>::deserialize(deserializer)?;
    match s {
        Some(ref val) => DateTime::parse_from_str(val, FORMAT)
            .map(|item| Some(item.to_utc()))
            .map_err(serde::de::Error::custom),
        None => Ok(None),
    }
}
