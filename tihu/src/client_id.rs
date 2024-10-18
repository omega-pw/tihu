use super::encoder;
use super::version_data;
use super::LightString;
use chrono::DateTime;
use chrono::Utc;
use log;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientIdV1 {
    rsa_pub_key: LightString,
    expire_time: i64, //过期时间，单位：秒
}

pub enum ClientId {
    V1(ClientIdV1),
}

impl ClientIdV1 {
    pub fn expired(&self) -> bool {
        let curr_time = Utc::now();
        return curr_time >= DateTime::from_timestamp(self.expire_time, 0).unwrap();
    }
}

impl ClientId {
    pub fn new(rsa_pub_key: LightString, expire_time: DateTime<Utc>) -> ClientId {
        ClientId::V1(ClientIdV1 {
            rsa_pub_key,
            expire_time: expire_time.timestamp(),
        })
    }

    fn version(&self) -> u64 {
        match self {
            ClientId::V1(_) => 1,
        }
    }

    pub fn rsa_pub_key(&self) -> &LightString {
        match self {
            ClientId::V1(client_id) => &client_id.rsa_pub_key,
        }
    }

    pub fn expire_time(&self) -> i64 {
        match self {
            ClientId::V1(client_id) => client_id.expire_time,
        }
    }

    pub fn expired(&self) -> bool {
        match self {
            ClientId::V1(client_id) => client_id.expired(),
        }
    }

    pub fn try_decode(
        client_id_data: &str,
        check_signature: impl FnOnce(&str, &[u8], &[u8]) -> Result<bool, LightString>,
    ) -> Result<ClientId, LightString> {
        let (version, chunk) = version_data::try_decode(client_id_data)?;
        match version {
            1 => {
                let (client_id_data, signature) = encoder::decode_chunks::<2>(chunk)?;
                let client_id: ClientIdV1 =
                    serde_json::from_slice(&client_id_data).map_err(|err| {
                        log::error!("反序列化客户端身份数据失败: {}", err);
                        return LightString::from_static("反序列化客户端身份数据失败!");
                    })?;
                if client_id.expired() {
                    return Err(LightString::from_static("客户端身份数据已过期!"));
                }
                if check_signature(&client_id.rsa_pub_key, &client_id_data, &signature)? {
                    return Ok(ClientId::V1(client_id));
                } else {
                    return Err(LightString::from_static("客户端身份数据签名不正确！"));
                }
            }
            _ => {
                return Err(LightString::from_static("未知的客户端身份数据版本!"));
            }
        }
    }

    pub fn encode<B>(
        &self,
        sign: impl FnOnce(&[u8]) -> Result<B, LightString>,
    ) -> Result<String, LightString>
    where
        B: AsRef<[u8]>,
    {
        let client_id_data = match self {
            ClientId::V1(client_id) => serde_json::to_vec(&client_id).map_err(|err| {
                log::error!("序列化客户端身份数据失败: {}", err);
                return LightString::from_static("序列化客户端身份数据失败!");
            })?,
        };
        let signature = sign(&client_id_data)?;
        return version_data::encode(
            self.version(),
            &encoder::encode_chunks(&[&client_id_data, signature.as_ref()], None),
        );
    }
}
