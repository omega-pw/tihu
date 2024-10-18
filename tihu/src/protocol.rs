use super::LightString;
use bytes::Bytes;
use bytes::BytesMut;
use integer_encoding::VarInt;
use std::convert::TryInto;

pub fn read_const_n<const N: usize>(mut input: Bytes) -> Result<([u8; N], Bytes), LightString> {
    let min_len = std::mem::size_of::<[u8; N]>();
    let mut output = [0; N];
    if input.len() < min_len {
        return Err(LightString::from_static("输入数据长度不够"));
    } else {
        let rest = input.split_off(min_len);
        output.copy_from_slice(&input);
        return Ok((output, rest));
    }
}

pub fn read_n(mut input: Bytes, n: usize) -> Result<(Bytes, Bytes), LightString> {
    let min_len = n;
    if input.len() < min_len {
        return Err(LightString::from_static("输入数据长度不够"));
    } else {
        let rest = input.split_off(min_len);
        return Ok((input, rest));
    }
}

pub fn read_u8(mut input: Bytes) -> Result<(u8, Bytes), LightString> {
    let min_len = 1;
    if input.len() < min_len {
        return Err(LightString::from_static("输入数据长度不够"));
    } else {
        let rest = input.split_off(min_len);
        return Ok((input[0], rest));
    }
}

pub fn read_be_u128(mut input: Bytes) -> Result<(u128, Bytes), LightString> {
    let min_len = std::mem::size_of::<u128>();
    if input.len() < min_len {
        return Err(LightString::from_static("输入数据长度不够"));
    } else {
        let rest = input.split_off(min_len);
        return Ok((
            u128::from_be_bytes(
                input
                    .as_ref()
                    .try_into()
                    .map_err(|err| -> LightString { format!("{:?}", err).into() })?,
            ),
            rest,
        ));
    }
}

pub fn read_be_i128(mut input: Bytes) -> Result<(i128, Bytes), LightString> {
    let min_len = std::mem::size_of::<i128>();
    if input.len() < min_len {
        return Err(LightString::from_static("输入数据长度不够"));
    } else {
        let rest = input.split_off(min_len);
        return Ok((
            i128::from_be_bytes(
                input
                    .as_ref()
                    .try_into()
                    .map_err(|err| -> LightString { format!("{:?}", err).into() })?,
            ),
            rest,
        ));
    }
}

pub fn read_be_u64(mut input: Bytes) -> Result<(u64, Bytes), LightString> {
    let min_len = std::mem::size_of::<u64>();
    if input.len() < min_len {
        return Err(LightString::from_static("输入数据长度不够"));
    } else {
        let rest = input.split_off(min_len);
        return Ok((
            u64::from_be_bytes(
                input
                    .as_ref()
                    .try_into()
                    .map_err(|err| -> LightString { format!("{:?}", err).into() })?,
            ),
            rest,
        ));
    }
}

pub fn read_be_i64(mut input: Bytes) -> Result<(i64, Bytes), LightString> {
    let min_len = std::mem::size_of::<i64>();
    if input.len() < min_len {
        return Err(LightString::from_static("输入数据长度不够"));
    } else {
        let rest = input.split_off(min_len);
        return Ok((
            i64::from_be_bytes(
                input
                    .as_ref()
                    .try_into()
                    .map_err(|err| -> LightString { format!("{:?}", err).into() })?,
            ),
            rest,
        ));
    }
}

pub trait Decoder {
    fn next(&mut self) -> Option<Bytes>;
    fn append(&mut self, extend: &[u8]);
}

pub trait Layer {
    type Decoder: Decoder;
    fn new_decoder() -> Self::Decoder;
    fn encode(buffer: &mut Vec<u8>, message: &[u8]);
}

/// This is the max required bytes to encode a u64 using the varint encoding scheme.
/// It is size 10=ceil(64/7)
const MAX_ENCODED_SIZE: usize = 10;

/// Encode a message, returning the bytes that must be sent before the message.
/// A buffer is used to avoid heap allocation.
fn encode_size<'a>(message: &[u8], buf: &'a mut [u8; MAX_ENCODED_SIZE]) -> &'a [u8] {
    let varint_size = message.len().encode_var(buf);
    &buf[..varint_size]
}

pub struct FramedLayer {}

impl Layer for FramedLayer {
    type Decoder = FramedState;
    fn new_decoder() -> Self::Decoder {
        FramedState::new()
    }
    fn encode(buffer: &mut Vec<u8>, message: &[u8]) {
        let mut buf = [0; MAX_ENCODED_SIZE];
        buffer.extend_from_slice(&*encode_size(message, &mut buf));
        buffer.extend_from_slice(message);
    }
}

pub struct FramedState {
    next_len: Option<usize>,
    data: BytesMut,
}

impl FramedState {
    pub fn new() -> FramedState {
        FramedState {
            next_len: None,
            data: BytesMut::new(),
        }
    }
}

impl Decoder for FramedState {
    fn next(&mut self) -> Option<Bytes> {
        loop {
            if let Some(next_len) = self.next_len {
                if 0 == next_len {
                    //长度为0，后面没有跟随数据
                    self.next_len = None;
                    //此处继续检查
                } else {
                    if next_len <= self.data.len() {
                        //buf的长度够了
                        let next = self.data.split_to(next_len);
                        self.next_len = None;
                        return Some(next.into());
                    } else {
                        //不够
                        return None;
                    }
                }
            } else {
                if let Some((next_len, cost)) = usize::decode_var(&self.data) {
                    let _ = self.data.split_to(cost);
                    self.next_len.replace(next_len);
                    //此处继续检查
                } else {
                    //buf的长度还不够长度的大小
                    return None;
                }
            }
        }
    }

    fn append(&mut self, extend: &[u8]) {
        self.data.extend_from_slice(extend);
    }
}
