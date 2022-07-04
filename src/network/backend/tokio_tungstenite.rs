use crate::{
  network::{Transport, TransportParams, TransportWrapper},
  Api,
};
use bytes::Bytes;
use core::fmt::Debug;
use futures::{SinkExt, StreamExt};
use serde::Serialize;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tungstenite::Message;

pub type TokioTungsteniteWsTransport = TransportWrapper<WebSocketStream<MaybeTlsStream<TcpStream>>>;
pub type TokioTungsteniteMutWsTransport<'backend> =
  TransportWrapper<&'backend mut WebSocketStream<MaybeTlsStream<TcpStream>>>;

impl TokioTungsteniteWsTransport {
  #[inline]
  pub async fn with_tokio_tungstenite<A>(api: &A) -> crate::Result<Self>
  where
    A: Api,
  {
    let (backend, _) = tokio_tungstenite::connect_async(api.origin().as_str()).await?;
    Ok(Self { backend })
  }
}

#[async_trait::async_trait]
impl Transport for TokioTungsteniteWsTransport {
  #[inline]
  async fn send<T>(&mut self, req: T, tp: &mut TransportParams) -> crate::Result<()>
  where
    T: Debug + Send + Serialize + Sync,
  {
    let mut this = TokioTungsteniteMutWsTransport { backend: &mut self.backend };
    <TokioTungsteniteMutWsTransport<'_>>::send(&mut this, req, tp).await
  }

  #[inline]
  async fn send_and_retrieve<T>(&mut self, req: T, tp: &mut TransportParams) -> crate::Result<Bytes>
  where
    T: Debug + Send + Serialize + Sync,
  {
    let mut this = TokioTungsteniteMutWsTransport { backend: &mut self.backend };
    <TokioTungsteniteMutWsTransport<'_>>::send_and_retrieve(&mut this, req, tp).await
  }
}

#[async_trait::async_trait]
impl Transport for TokioTungsteniteMutWsTransport<'_> {
  #[inline]
  async fn send<T>(&mut self, req: T, tp: &mut TransportParams) -> crate::Result<()>
  where
    T: Debug + Send + Serialize + Sync,
  {
    self.backend.send(Message::Binary(serde_json::to_vec(&req)?)).await?;
    tp._clear();
    Ok(())
  }

  #[inline]
  async fn send_and_retrieve<T>(&mut self, req: T, tp: &mut TransportParams) -> crate::Result<Bytes>
  where
    T: Debug + Send + Serialize + Sync,
  {
    self.send(req, tp).await?;
    tp._clear();
    Ok(if let Some(elem) = self.backend.next().await {
      elem?.into_data().into()
    } else {
      Default::default()
    })
  }
}
