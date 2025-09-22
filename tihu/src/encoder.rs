use super::tuple::TupleMany;
use super::tuple::TupleManyStruct;
use super::tuple::TupleManyTrait;
use super::SharedString;
use bytes::Bytes;
use integer_encoding::VarInt;

/// This is the max required bytes to encode a u64 using the varint encoding scheme.
/// It is size 10=ceil(64/7)
const MAX_ENCODED_SIZE: usize = 10;

/// Encode a message, returning the bytes that must be sent before the message.
/// A buffer is used to avoid heap allocation.
fn encode_size<'a>(message: &[u8], buf: &'a mut [u8; MAX_ENCODED_SIZE]) -> &'a [u8] {
    let varint_size = message.len().encode_var(buf);
    &buf[..varint_size]
}

pub fn encode_chunks(chunks: &[&[u8]], output: Option<Vec<u8>>) -> Vec<u8> {
    if chunks.is_empty() {
        return Vec::new();
    }
    let count = chunks.len();
    let mut output = output.unwrap_or_else(|| {
        let max_len =
            (count - 1) * MAX_ENCODED_SIZE + chunks.iter().map(|chunk| chunk.len()).sum::<usize>();
        Vec::with_capacity(max_len)
    });
    let mut buf = [0; MAX_ENCODED_SIZE];
    for (index, chunk) in chunks.iter().enumerate() {
        if index + 1 < count {
            output.extend_from_slice(&*encode_size(chunk, &mut buf));
        }
        output.extend_from_slice(chunk);
    }
    return output;
}

pub fn decode_chunks<const N: usize>(mut chunk: Bytes) -> Result<TupleMany<N, Bytes>, SharedString>
where
    TupleManyStruct<N>: TupleManyTrait<Bytes>,
{
    let mut count: usize = N;
    let mut chunks: Vec<Bytes> = Vec::with_capacity(count);
    while 1 < count {
        let (size, cost) = usize::decode_var(&chunk)
            .ok_or_else(|| SharedString::from_static("输入数据长度不够"))?;
        let mut remain = chunk.split_off(cost);
        if remain.len() < size {
            return Err(SharedString::from_static("输入数据长度不够"));
        } else {
            chunk = remain.split_off(size);
            chunks.push(remain);
            count -= 1;
        }
    }
    chunks.push(chunk);
    return TupleManyStruct::try_from_iter(chunks.into_iter())
        .map_err(|_| SharedString::from_static("数据元素个数不正确"));
}
