use crate::{
  api::Api,
  network::{Transport, TransportParams},
};
use bytes::Bytes;
use core::fmt::Debug;
use futures::{SinkExt, StreamExt};
use serde::Serialize;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tungstenite::Message;

/// Shortcut of `WebSocketStream<MaybeTlsStream<TcpStream>>`.
pub type TokioTungstenite = WebSocketStream<MaybeTlsStream<TcpStream>>;

type TokioTungsteniteMut<'transport> = &'transport mut TokioTungstenite;

/// Handy constructor to avoid having to explicitly import `tokio_tungstenite` dependencies.
#[inline]
pub async fn tokio_tungstenite<A>(api: &A) -> crate::Result<TokioTungstenite>
where
  A: Api,
{
  let (elem, _) = tokio_tungstenite::connect_async(api.origin().as_str()).await?;
  Ok(elem)
}

#[async_trait::async_trait]
impl Transport for WebSocketStream<MaybeTlsStream<TcpStream>> {
  #[inline]
  async fn send<T>(&mut self, req: T, tp: &mut TransportParams) -> crate::Result<()>
  where
    T: Debug + Send + Serialize + Sync,
  {
    <Self as Transport>::send(self, req, tp).await
  }

  #[inline]
  async fn send_and_retrieve<T>(&mut self, req: T, tp: &mut TransportParams) -> crate::Result<Bytes>
  where
    T: Debug + Send + Serialize + Sync,
  {
    <Self as Transport>::send_and_retrieve(self, req, tp).await
  }
}

#[async_trait::async_trait]
impl Transport for TokioTungsteniteMut<'_> {
  #[inline]
  async fn send<T>(&mut self, req: T, tp: &mut TransportParams) -> crate::Result<()>
  where
    T: Debug + Send + Serialize + Sync,
  {
    <Self as SinkExt<_>>::send(self, Message::Binary(serde_json::to_vec(&req)?)).await?;
    tp._clear();
    Ok(())
  }

  #[inline]
  async fn send_and_retrieve<T>(&mut self, req: T, tp: &mut TransportParams) -> crate::Result<Bytes>
  where
    T: Debug + Send + Serialize + Sync,
  {
    <Self as Transport>::send(self, req, tp).await?;
    tp._clear();
    Ok(if let Some(elem) = self.next().await {
      elem?.into_data().into()
    } else {
      Default::default()
    })
  }
}
