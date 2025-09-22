use super::SharedString;
use crate::Bytes;
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::convert::AsRef;

pub type ApiResult<Output, CallError> = Result<Response<Output>, ApiErr<CallError>>;

#[derive(thiserror::Error, Debug)]
pub enum ApiErr<E> {
    #[error("序列化数据失败,{0}")]
    SerializeError(serde_json::Error),
    #[error("反序列化数据失败,{0}")]
    DeserializeError(serde_json::Error),
    #[error("调用远程接口失败,{0}")]
    CallError(E),
}

#[async_trait]
pub trait ApiClient {
    type Output: AsRef<[u8]>;
    type Error;
    async fn request(&self, namespace: &str, input: Bytes) -> Result<Self::Output, Self::Error>;
}

#[async_trait]
pub trait Api {
    type Input;
    type Output;
    fn namespace() -> SharedString;
    fn require_res_key() -> Option<SharedString> {
        return None;
    }
    fn validate_input(_: &Self::Input) -> Result<(), SharedString> {
        return Ok(());
    }
    async fn call<Client, ClientOutput, E>(
        &self,
        client: &Client,
        input: &Self::Input,
    ) -> Result<Response<Self::Output>, ApiErr<E>>
    where
        Client: ApiClient<Error = E, Output = ClientOutput> + Sync,
        ClientOutput: AsRef<[u8]>,
        Self::Input: Serialize + Sync,
        Self::Output: DeserializeOwned + 'static,
    {
        let input = serde_json::to_vec(input).map_err(ApiErr::SerializeError)?;
        let output = client
            .request(&Self::namespace(), input.into())
            .await
            .map_err(ApiErr::CallError)?;
        let response = serde_json::from_slice::<Response<Self::Output>>(output.as_ref())
            .map_err(ApiErr::DeserializeError)?;
        return Ok(response);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response<T> {
    pub code: i32,
    pub data: Option<T>,
    pub message: SharedString,
}

impl<T> Response<T> {
    pub fn success(data: Option<T>) -> Response<T> {
        return Response {
            code: 0,
            data: data,
            message: SharedString::from_static("success"),
        };
    }
    pub fn failure(mut code: i32, msg: SharedString, data: Option<T>) -> Response<T> {
        if 0 == code {
            code = -1;
        }
        return Response {
            code: code,
            data: data,
            message: msg,
        };
    }
}

pub fn success() -> &'static [u8] {
    return b"{\"code\":0,\"message\":\"success\",\"data\":null}";
}
