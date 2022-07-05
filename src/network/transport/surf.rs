use crate::network::{HttpMethod, Transport, TransportParams};
use alloc::boxed::Box;
use bytes::Bytes;
use core::fmt::Debug;
use serde::Serialize;
use surf::{Client, Response};

/// Handy constructor to avoid having to explicitly import `surf` dependencies.
#[inline]
pub fn surf() -> Client {
  <_>::default()
}

#[async_trait::async_trait]
impl Transport for Client {
  #[inline]
  async fn send<T>(&mut self, req: T, tp: &mut TransportParams) -> crate::Result<()>
  where
    T: Debug + Send + Serialize + Sync,
  {
    let _ = reqwest_send(self, req, tp).await;
    Ok(())
  }

  #[inline]
  async fn send_and_retrieve<T>(&mut self, req: T, tp: &mut TransportParams) -> crate::Result<Bytes>
  where
    T: Debug + Send + Serialize + Sync,
  {
    Ok(reqwest_send(self, req, tp).await?.body_bytes().await?.into())
  }
}

#[async_trait::async_trait]
impl Transport for &Client {
  #[inline]
  async fn send<T>(&mut self, req: T, tp: &mut TransportParams) -> crate::Result<()>
  where
    T: Debug + Send + Serialize + Sync,
  {
    let _ = reqwest_send(self, req, tp).await;
    Ok(())
  }

  #[inline]
  async fn send_and_retrieve<T>(&mut self, req: T, tp: &mut TransportParams) -> crate::Result<Bytes>
  where
    T: Debug + Send + Serialize + Sync,
  {
    Ok(reqwest_send(self, req, tp).await?.body_bytes().await?.into())
  }
}

#[inline]
async fn reqwest_send<T>(
  client: &Client,
  req: T,
  tp: &mut TransportParams,
) -> crate::Result<Response>
where
  T: Debug + Send + Serialize + Sync,
{
  if let Some(ref mut rt) = tp._http_params._rt {
    rt.rc.update_params(&rt.rl).await;
  }
  let rslt = match tp._http_params._method {
    HttpMethod::Get => client.get(tp._http_params._url.as_str()).await,
    HttpMethod::Post => client.post(tp._http_params._url.as_str()).body_json(&req)?.await,
  };
  tp._clear();
  Ok(rslt?)
}
