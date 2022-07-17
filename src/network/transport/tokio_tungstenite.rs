use crate::{
  network::{Transport, WsParams},
  RequestManager, RequestParams,
};
use alloc::vec::Vec;
use core::fmt::Debug;
use futures::{SinkExt, StreamExt};
use serde::Serialize;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tungstenite::Message;

/// Shortcut of `WebSocketStream<MaybeTlsStream<TcpStream>>`.
pub type TokioTungstenite = WebSocketStream<MaybeTlsStream<TcpStream>>;

/// Handy constructor to avoid having to explicitly import `tokio-tungstenite`.
#[inline]
pub async fn tokio_tungstenite(ws_params: &WsParams) -> crate::Result<TokioTungstenite> {
  let (elem, _) = tokio_tungstenite::connect_async(ws_params._url_parts.origin()).await?;
  Ok(elem)
}

/// Handy constructor to avoid having to explicitly import `tokio_tungstenite` dependencies.
///
/// ```rust,no_run
/// # async fn fun() -> lucia::Result<()> {
/// use lucia::{
///   network::{tokio_tungstenite, Transport, WsParams},
///   Pair,
/// };
/// let ws_params = WsParams::from_origin("ORIGIN")?;
/// let (mut rm, mut trans) =
///   Pair::<(), _, _>::new(tokio_tungstenite(&ws_params).await?, ws_params).into_parts();
/// let req = ();
/// let _res = trans.send_retrieve_and_decode_one(&mut rm, &req, ()).await?;
/// Ok(())
/// # }
/// ```
#[async_trait::async_trait]
impl<A> Transport<A, WsParams> for TokioTungstenite
where
  A: Send,
{
  #[inline]
  async fn send<R, RPD>(
    &mut self,
    _: &mut RequestManager<A, WsParams>,
    req: R,
    _: RPD,
  ) -> crate::Result<()>
  where
    R: Debug + RequestParams<WsParams, RPD> + Send + Serialize + Sync,
    RPD: Send,
  {
    <Self as SinkExt<_>>::send(self, Message::Binary(serde_json::to_vec(&req)?)).await?;
    Ok(())
  }

  #[inline]
  async fn send_and_retrieve<R, RPD>(
    &mut self,
    _: &mut RequestManager<A, WsParams>,
    req: R,
    _: RPD,
  ) -> crate::Result<Vec<u8>>
  where
    R: Debug + RequestParams<WsParams, RPD> + Send + Serialize + Sync,
    RPD: Send,
  {
    <Self as SinkExt<_>>::send(self, Message::Binary(serde_json::to_vec(&req)?)).await?;
    Ok(if let Some(elem) = self.next().await { elem?.into_data().into() } else { Vec::new() })
  }
}
