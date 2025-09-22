use bytes::Bytes;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::ops::Deref;
use std::sync::Arc;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum SharedString {
    Static(&'static str),
    Arc(Arc<str>),
}

impl SharedString {
    pub const fn from_static(src: &'static str) -> SharedString {
        SharedString::Static(src)
    }
    #[inline]
    pub fn as_str(&self) -> &str {
        match self {
            SharedString::Static(val) => val,
            SharedString::Arc(val) => val.as_ref(),
        }
    }
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.as_str().as_bytes()
    }
    pub fn into_bytes(self) -> Bytes {
        match self {
            SharedString::Static(val) => Bytes::from_static(val.as_bytes()),
            SharedString::Arc(val) => Bytes::from(val.as_bytes().to_vec()),
        }
    }
}

impl Default for SharedString {
    fn default() -> Self {
        SharedString::from_static("")
    }
}

impl AsRef<str> for SharedString {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Deref for SharedString {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for SharedString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.deref(), f)
    }
}

impl From<&'static str> for SharedString {
    #[inline]
    fn from(value: &'static str) -> Self {
        SharedString::from_static(value)
    }
}

impl From<String> for SharedString {
    #[inline]
    fn from(value: String) -> Self {
        SharedString::Arc(Arc::from(value))
    }
}

impl From<Arc<str>> for SharedString {
    #[inline]
    fn from(value: Arc<str>) -> Self {
        SharedString::Arc(value)
    }
}

impl From<Box<str>> for SharedString {
    #[inline]
    fn from(value: Box<str>) -> Self {
        SharedString::Arc(Arc::from(value))
    }
}

impl Serialize for SharedString {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_ref())
    }
}

impl<'de> Deserialize<'de> for SharedString {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer).map(SharedString::from)
    }
}

impl std::error::Error for SharedString {}
