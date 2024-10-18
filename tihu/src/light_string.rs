use bytes::Bytes;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::ops::Deref;
use std::sync::Arc;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LightString {
    Static(&'static str),
    Arc(Arc<str>),
}

impl LightString {
    pub const fn from_static(src: &'static str) -> LightString {
        LightString::Static(src)
    }
    #[inline]
    pub fn as_str(&self) -> &str {
        match self {
            LightString::Static(val) => val,
            LightString::Arc(val) => val.as_ref(),
        }
    }
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.as_str().as_bytes()
    }
    pub fn into_bytes(self) -> Bytes {
        match self {
            LightString::Static(val) => Bytes::from_static(val.as_bytes()),
            LightString::Arc(val) => Bytes::from(val.as_bytes().to_vec()),
        }
    }
}

impl Default for LightString {
    fn default() -> Self {
        LightString::from_static("")
    }
}

impl AsRef<str> for LightString {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Deref for LightString {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for LightString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.deref(), f)
    }
}

impl From<&'static str> for LightString {
    #[inline]
    fn from(value: &'static str) -> Self {
        LightString::from_static(value)
    }
}

impl From<String> for LightString {
    #[inline]
    fn from(value: String) -> Self {
        LightString::Arc(Arc::from(value))
    }
}

impl From<Arc<str>> for LightString {
    #[inline]
    fn from(value: Arc<str>) -> Self {
        LightString::Arc(value)
    }
}

impl From<Box<str>> for LightString {
    #[inline]
    fn from(value: Box<str>) -> Self {
        LightString::Arc(Arc::from(value))
    }
}

impl Serialize for LightString {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_ref())
    }
}

impl<'de> Deserialize<'de> for LightString {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer).map(LightString::from)
    }
}

impl std::error::Error for LightString {}
