use crate::network::{HttpMethod, Transport, TransportParams, TransportWrapper};
use alloc::boxed::Box;
use bytes::Bytes;
use core::fmt::Debug;
use serde::Serialize;
use surf::{Client, Response};

pub type SurfHttpTransport = TransportWrapper<Client>;
pub type SurfRefHttpTransport<'backend> = TransportWrapper<&'backend Client>;

impl SurfHttpTransport {
  #[inline]
  pub fn with_surf() -> Self {
    Self { backend: Client::new() }
  }
}

#[async_trait::async_trait]
impl Transport for SurfHttpTransport {
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
    Ok(reqwest_send(&self.backend, req, tp).await?.body_bytes().await?.into())
  }
}

#[async_trait::async_trait]
impl Transport for SurfRefHttpTransport<'_> {
  #[inline]
  async fn send<T>(&mut self, req: T, tp: &mut TransportParams) -> crate::Result<()>
  where
    T: Debug + Send + Serialize + Sync,
  {
    let _ = reqwest_send(self.backend, req, tp).await;
    Ok(())
  }

  #[inline]
  async fn send_and_retrieve<T>(&mut self, req: T, tp: &mut TransportParams) -> crate::Result<Bytes>
  where
    T: Debug + Send + Serialize + Sync,
  {
    Ok(reqwest_send(self.backend, req, tp).await?.body_bytes().await?.into())
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
    HttpMethod::Get => backend.get(tp.http_params.url.as_str()).await,
    HttpMethod::Post => backend.post(tp.http_params.url.as_str()).body_json(&req)?.await,
  };
  tp._clear();
  Ok(rslt?)
}
