use crate::network::{HttpMethod, Transport, TransportParams, TransportWrapper};
use bytes::Bytes;
use core::fmt::Debug;
use reqwest::{Client, Response};
use serde::Serialize;

pub type ReqwestHttpTransport = TransportWrapper<Client>;
pub type ReqwestRefHttpTransport<'backend> = TransportWrapper<&'backend Client>;

impl ReqwestHttpTransport {
  #[inline]
  pub fn with_reqwest() -> Self {
    Self { backend: Client::new() }
  }
}

#[async_trait::async_trait]
impl Transport for ReqwestHttpTransport {
  #[inline]
  async fn send<T>(&mut self, req: T, tp: &mut TransportParams) -> crate::Result<()>
  where
    T: Debug + Send + Serialize + Sync,
  {
    let _ = reqwest_send(&self.backend, req, tp).await;
    Ok(())
  }

  #[inline]
  async fn send_and_retrieve<T>(&mut self, req: T, tp: &mut TransportParams) -> crate::Result<Bytes>
  where
    T: Debug + Send + Serialize + Sync,
  {
    Ok(reqwest_send(&self.backend, req, tp).await?.bytes().await?)
  }
}

#[async_trait::async_trait]
impl Transport for ReqwestRefHttpTransport<'_> {
  #[inline]
  async fn send<T>(&mut self, req: T, tp: &mut TransportParams) -> crate::Result<()>
  where
    T: Debug + Send + Serialize + Sync,
  {
    let _ = reqwest_send(self.backend, req, tp).await?;
    Ok(())
  }

  #[inline]
  async fn send_and_retrieve<T>(&mut self, req: T, tp: &mut TransportParams) -> crate::Result<Bytes>
  where
    T: Debug + Send + Serialize + Sync,
  {
    Ok(reqwest_send(self.backend, req, tp).await?.bytes().await?)
  }
}

#[inline]
async fn reqwest_send<T>(
  backend: &Client,
  req: T,
  tp: &mut TransportParams,
) -> crate::Result<Response>
where
  T: Debug + Send + Serialize + Sync,
{
  let rslt = match tp.http_params.method {
    HttpMethod::Get => backend.get(tp.http_params.url.as_str()).send().await,
    HttpMethod::Post => backend.post(tp.http_params.url.as_str()).json(&req).send().await,
  };
  tp._clear();
  Ok(rslt?)
}
