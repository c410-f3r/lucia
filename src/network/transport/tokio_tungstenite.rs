use crate::{
  dnsn::Serialize,
  network::{
    ws::{ReqParams, ReqParamsMut},
    Transport,
  },
  RequestManager, RequestParamsModifier,
};
use alloc::vec::Vec;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tungstenite::Message;

/// Shortcut of `WebSocketStream<MaybeTlsStream<TcpStream>>`.
pub type TokioTungstenite = WebSocketStream<MaybeTlsStream<TcpStream>>;

/// Handy constructor to avoid having to explicitly import `tokio-tungstenite`.
#[inline]
pub async fn tokio_tungstenite(ws_params: &ReqParams) -> crate::Result<TokioTungstenite> {
  let (elem, _) = tokio_tungstenite::connect_async(ws_params._url_parts.origin()).await?;
  Ok(elem)
}

/// Handy constructor to avoid having to explicitly import `tokio_tungstenite` dependencies.
///
/// ```rust,no_run
/// # async fn fun() -> lucia::Result<()> {
/// use lucia::{
///   network::{tokio_tungstenite, Transport, ws::ReqParams},
///   CommonParams, Pair, RequestManager
/// };
/// let req_params = ReqParams::from_origin("ORIGIN")?;
/// let tokio_tungstenite = tokio_tungstenite(&req_params).await?;
/// let (mut rm, mut trans) = Pair::new(
///   RequestManager::new((), CommonParams::new(req_params, ()), ()),
///   tokio_tungstenite,
/// ).into_parts();
/// let req = ();
/// let _res = trans.send_retrieve_and_decode_one(&mut rm, &req, ()).await?;
/// # Ok(()) }
/// ```
#[async_trait::async_trait]
impl<A, CP, DRSR> Transport<A, CP, DRSR> for TokioTungstenite
where
  A: Send,
  CP: Send,
  DRSR: Send,
  for<'any> &'any mut CP: Into<ReqParamsMut<'any>>,
{
  type Metadata = ();

  #[inline]
  async fn send<REQ, REQP>(
    &mut self,
    rm: &mut RequestManager<A, CP, DRSR>,
    req: &REQ,
    _: REQP,
  ) -> crate::Result<()>
  where
    REQ: RequestParamsModifier<CP, REQP> + Send + Serialize<DRSR> + Sync,
    REQP: Send,
  {
    let mut vec = Vec::new();
    req.to_bytes(&mut vec, &mut rm.drsr)?;
    <Self as SinkExt<_>>::send(self, Message::Binary(vec)).await?;
    Ok(())
  }

  #[inline]
  async fn send_and_retrieve<REQ, REQP>(
    &mut self,
    rm: &mut RequestManager<A, CP, DRSR>,
    req: &REQ,
    _: REQP,
  ) -> crate::Result<(Self::Metadata, Vec<u8>)>
  where
    REQ: RequestParamsModifier<CP, REQP> + Send + Serialize<DRSR> + Sync,
    REQP: Send,
  {
    let mut vec = Vec::new();
    req.to_bytes(&mut vec, &mut rm.drsr)?;
    <Self as SinkExt<_>>::send(self, Message::Binary(vec)).await?;
    Ok(if let Some(elem) = self.next().await {
      ((), elem?.into_data().into())
    } else {
      ((), Vec::new())
    })
  }
}
