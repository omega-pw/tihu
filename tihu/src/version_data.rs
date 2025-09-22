use super::base62;
use super::protocol::read_be_u64;
use super::SharedString;
use bytes::Bytes;

pub fn try_decode(data: &str) -> Result<(u64, Bytes), SharedString> {
    let data = base62::decode(data).map_err(|err| SharedString::from(err.to_string()))?;
    return read_be_u64(data.into());
}

pub fn encode(version: u64, data: &[u8]) -> Result<String, SharedString> {
    let version = version.to_be_bytes();
    let chunk = [&version, data].concat();
    let data = base62::encode(&chunk);
    return Ok(data);
}
