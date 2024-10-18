use async_trait::async_trait;
use bytes::Bytes;
pub use http_body_util::combinators::BoxBody;
use hyper::body::Incoming;
use hyper::{Request, Response};
use std::net::SocketAddr;

#[async_trait]
pub trait HttpHandler: Sync + Send + 'static {
    fn namespace(&self) -> Vec<String>;
    async fn handle(
        &self,
        request: Request<Incoming>,
        remote_addr: SocketAddr,
    ) -> Result<Response<BoxBody<Bytes, anyhow::Error>>, hyper::Error>;
}
