use crate::network::{HttpMethod, Transport, TransportParams};
use bytes::Bytes;
use core::fmt::Debug;
use reqwest::{Client, Response};
use serde::Serialize;

/// Handy constructor to avoid having to explicitly import `reqwest` dependencies.
#[inline]
pub fn reqwest() -> Client {
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
    Ok(reqwest_send(self, req, tp).await?.bytes().await?)
  }
}

#[async_trait::async_trait]
impl Transport for &Client {
  #[inline]
  async fn send<T>(&mut self, req: T, tp: &mut TransportParams) -> crate::Result<()>
  where
    T: Debug + Send + Serialize + Sync,
  {
    let _ = reqwest_send(self, req, tp).await?;
    Ok(())
  }

  #[inline]
  async fn send_and_retrieve<T>(&mut self, req: T, tp: &mut TransportParams) -> crate::Result<Bytes>
  where
    T: Debug + Send + Serialize + Sync,
  {
    Ok(reqwest_send(self, req, tp).await?.bytes().await?)
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
    HttpMethod::Get => client.get(tp._http_params._url.as_str()).send().await,
    HttpMethod::Post => client.post(tp._http_params._url.as_str()).json(&req).send().await,
  };
  tp._clear();
  Ok(rslt?)
}
