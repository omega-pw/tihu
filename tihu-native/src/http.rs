use async_trait::async_trait;
use bytes::Bytes;
use bytes::BytesMut;
use futures::Stream;
use futures::StreamExt;
use futures::TryStreamExt;
use headers::Cookie;
use headers::HeaderMapExt;
use http::Extensions;
use http_body_util::BodyExt;
use hyper::body::Frame;
use hyper::body::Incoming;
use hyper::header::HeaderValue;
use hyper::HeaderMap;
use hyper::Method;
use hyper::Uri;
use hyper::Version;
use hyper::{Request, Response};
use pin_project::pin_project;
use std::any::Any;
use std::any::TypeId;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::net::SocketAddr;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use sync_wrapper::SyncStream;
use tihu::LightString;

pub type BoxBody = http_body_util::combinators::BoxBody<Bytes, anyhow::Error>;

/// A body object for requests and responses.
#[derive(Default)]
#[pin_project]
pub struct Body(#[pin] pub(crate) BoxBody);

impl From<Body> for BoxBody {
    #[inline]
    fn from(body: Body) -> Self {
        body.0
    }
}

impl From<BoxBody> for Body {
    #[inline]
    fn from(body: BoxBody) -> Self {
        Body(body)
    }
}

impl Debug for Body {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Body").finish()
    }
}

impl From<&'static [u8]> for Body {
    #[inline]
    fn from(data: &'static [u8]) -> Self {
        Self(BoxBody::new(
            http_body_util::Full::new(data.into()).map_err::<_, anyhow::Error>(|_| unreachable!()),
        ))
    }
}

impl From<&'static str> for Body {
    #[inline]
    fn from(data: &'static str) -> Self {
        Self(BoxBody::new(
            http_body_util::Full::new(data.into()).map_err::<_, anyhow::Error>(|_| unreachable!()),
        ))
    }
}

impl From<Bytes> for Body {
    #[inline]
    fn from(data: Bytes) -> Self {
        Self(
            http_body_util::Full::new(data)
                .map_err::<_, anyhow::Error>(|_| unreachable!())
                .boxed(),
        )
    }
}

impl From<Vec<u8>> for Body {
    #[inline]
    fn from(data: Vec<u8>) -> Self {
        Self(
            http_body_util::Full::new(data.into())
                .map_err::<_, anyhow::Error>(|_| unreachable!())
                .boxed(),
        )
    }
}

impl From<Cow<'static, [u8]>> for Body {
    #[inline]
    fn from(data: Cow<'static, [u8]>) -> Self {
        Self(
            http_body_util::Full::from(data)
                .map_err::<_, anyhow::Error>(|_| unreachable!())
                .boxed(),
        )
    }
}

impl From<String> for Body {
    #[inline]
    fn from(data: String) -> Self {
        data.into_bytes().into()
    }
}

impl From<LightString> for Body {
    #[inline]
    fn from(data: LightString) -> Self {
        match data {
            LightString::Arc(data) => Body::from(data.to_string()),
            LightString::Static(data) => Body::from(data),
        }
    }
}

impl From<()> for Body {
    #[inline]
    fn from(_: ()) -> Self {
        Body::empty()
    }
}

impl Body {
    /// Create a body object from [`Bytes`].
    #[inline]
    pub fn from_bytes(data: Bytes) -> Self {
        data.into()
    }

    /// Create a body object from [`String`].
    #[inline]
    pub fn from_string(data: String) -> Self {
        data.into()
    }

    /// Create a body object from bytes stream.
    pub fn from_bytes_stream<S, O, E>(stream: S) -> Self
    where
        S: Stream<Item = Result<O, E>> + Send + 'static,
        O: Into<Bytes> + 'static,
        E: Into<anyhow::Error> + 'static,
    {
        Self(BoxBody::new(http_body_util::StreamBody::new(
            SyncStream::new(
                stream
                    .map_ok(|data| Frame::data(data.into()))
                    .map_err(Into::into),
            ),
        )))
    }

    /// Create a body object from [`Vec<u8>`].
    #[inline]
    pub fn from_vec(data: Vec<u8>) -> Self {
        data.into()
    }

    /// Create an empty body.
    #[inline]
    pub fn empty() -> Self {
        Self(
            http_body_util::Empty::new()
                .map_err::<_, anyhow::Error>(|_| unreachable!())
                .boxed(),
        )
    }

    #[inline]
    pub fn into_inner(self) -> BoxBody {
        self.0
    }
}

impl hyper::body::Body for Body {
    type Data = Bytes;
    type Error = anyhow::Error;
    fn poll_frame(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        let this = self.project();
        hyper::body::Body::poll_frame(this.0, cx)
    }
}

pub fn body_to_stream<B>(
    mut body: B,
) -> impl Stream<Item = Result<hyper::body::Frame<Bytes>, anyhow::Error>>
where
    B: hyper::body::Body<Data = Bytes, Error = anyhow::Error> + Unpin,
{
    futures::stream::poll_fn(
        move |cx| -> std::task::Poll<Option<Result<hyper::body::Frame<Bytes>, anyhow::Error>>> {
            hyper::body::Body::poll_frame(std::pin::Pin::new(&mut body), cx)
        },
    )
}

pub async fn read_body<B>(body: B) -> Result<Bytes, anyhow::Error>
where
    B: hyper::body::Body<Data = Bytes, Error = anyhow::Error> + Unpin,
{
    let mut bytes = BytesMut::new();
    let mut stream = body_to_stream(body);
    while let Some(frame) = stream.next().await {
        let frame = frame?;
        if let Some(frame) = frame.data_ref() {
            bytes.extend_from_slice(frame);
        }
    }
    return Ok(bytes.into());
}

#[async_trait]
pub trait HttpHandler: Sync + Send + 'static {
    fn namespace(&self) -> &[LightString];
    async fn handle(
        &self,
        request: Request<Incoming>,
        remote_addr: SocketAddr,
        request_data: &mut RequestData,
        prefix: Option<&str>,
    ) -> Result<Response<BoxBody>, anyhow::Error>;
}

#[async_trait]
pub trait HttpAuthorizer: Sync + Send + 'static {
    async fn authorize(
        &self,
        request: &Request<Incoming>,
        remote_addr: SocketAddr,
        request_data: &mut RequestData,
        prefix: Option<&str>,
    ) -> Result<bool, anyhow::Error>;
}

#[async_trait]
pub trait FromRequest: Sync + Send + 'static {
    async fn try_extract(
        request: &Request<Incoming>,
        remote_addr: SocketAddr,
        request_data: &mut RequestData,
    ) -> Result<Self, anyhow::Error>
    where
        Self: Sized;
}

#[derive(Default)]
pub struct RequestData {
    data_map: HashMap<TypeId, Box<dyn Any + Sync + Send>>,
}

impl RequestData {
    pub fn new() -> Self {
        Default::default()
    }
}

impl RequestData {
    pub async fn try_get<T>(
        &mut self,
        request: &Request<Incoming>,
        remote_addr: SocketAddr,
    ) -> Result<&T, anyhow::Error>
    where
        T: FromRequest,
    {
        let type_id = TypeId::of::<T>();
        let exist = self.data_map.get(&type_id).is_some();
        if !exist {
            let data = T::try_extract(request, remote_addr, self).await?;
            self.data_map.insert(type_id, Box::new(data));
        }
        let data = self
            .data_map
            .get(&type_id)
            .ok_or_else(|| LightString::from_static("Data is empty!"))?;
        let data = data
            .downcast_ref::<T>()
            .ok_or_else(|| LightString::from_static("Data not match the type!"))?;
        return Ok(data);
    }
    pub fn remove<T>(&mut self) -> Result<Option<Box<T>>, anyhow::Error>
    where
        T: FromRequest,
    {
        let type_id = TypeId::of::<T>();
        if let Some(data) = self.data_map.remove(&type_id) {
            match data.downcast::<T>() {
                Ok(data) => {
                    return Ok(Some(data));
                }
                Err(data) => {
                    self.data_map.insert(type_id, Box::new(data));
                    return Err(LightString::from_static("Data not match the type!").into());
                }
            }
        } else {
            return Ok(None);
        }
    }
    pub async fn remove_or_get<T>(
        &mut self,
        request: &Request<Incoming>,
        remote_addr: SocketAddr,
    ) -> Result<T, anyhow::Error>
    where
        T: FromRequest,
    {
        let data_opt = self.remove::<T>()?;
        if let Some(data) = data_opt {
            return Ok(*data);
        } else {
            let data = T::try_extract(request, remote_addr, self).await?;
            return Ok(data);
        }
    }
}

#[async_trait]
impl FromRequest for Option<Cookie> {
    async fn try_extract(
        request: &Request<Incoming>,
        _remote_addr: SocketAddr,
        _request_data: &mut RequestData,
    ) -> Result<Self, anyhow::Error> {
        let cookie = request.headers().typed_get::<Cookie>();
        return Ok(cookie);
    }
}

#[async_trait]
impl FromRequest for Method {
    async fn try_extract(
        request: &Request<Incoming>,
        _remote_addr: SocketAddr,
        _request_data: &mut RequestData,
    ) -> Result<Self, anyhow::Error> {
        return Ok(request.method().clone());
    }
}

#[async_trait]
impl FromRequest for Uri {
    async fn try_extract(
        request: &Request<Incoming>,
        _remote_addr: SocketAddr,
        _request_data: &mut RequestData,
    ) -> Result<Self, anyhow::Error> {
        return Ok(request.uri().clone());
    }
}

#[async_trait]
impl FromRequest for Version {
    async fn try_extract(
        request: &Request<Incoming>,
        _remote_addr: SocketAddr,
        _request_data: &mut RequestData,
    ) -> Result<Self, anyhow::Error> {
        return Ok(request.version());
    }
}

#[async_trait]
impl FromRequest for HeaderMap<HeaderValue> {
    async fn try_extract(
        request: &Request<Incoming>,
        _remote_addr: SocketAddr,
        _request_data: &mut RequestData,
    ) -> Result<Self, anyhow::Error> {
        return Ok(request.headers().clone());
    }
}

#[async_trait]
impl FromRequest for Extensions {
    async fn try_extract(
        request: &Request<Incoming>,
        _remote_addr: SocketAddr,
        _request_data: &mut RequestData,
    ) -> Result<Self, anyhow::Error> {
        return Ok(request.extensions().clone());
    }
}
