use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::num::ParseIntError;
use std::ops::Deref;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, Hash)]
pub struct Id8(pub i64);

impl fmt::Display for Id8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq<i64> for Id8 {
    fn eq(&self, other: &i64) -> bool {
        self.0 == *other
    }
}

impl Serialize for Id8 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", self.0);
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for Id8 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        i64::from_str_radix(&s, 10)
            .map(Id8)
            .map_err(serde::de::Error::custom)
    }
}

impl From<i64> for Id8 {
    fn from(id: i64) -> Self {
        Id8(id)
    }
}

impl Into<i64> for Id8 {
    fn into(self) -> i64 {
        self.0
    }
}

impl Deref for Id8 {
    type Target = i64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<i64> for Id8 {
    fn as_ref(&self) -> &i64 {
        &self.0
    }
}

impl AsMut<i64> for Id8 {
    fn as_mut(&mut self) -> &mut i64 {
        &mut self.0
    }
}

impl FromStr for Id8 {
    type Err = ParseIntError;
    fn from_str(src: &str) -> Result<Self, ParseIntError> {
        i64::from_str(src).map(From::from)
    }
}

pub type Id = Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct PrimaryKey {
    pub id: Id,
}
