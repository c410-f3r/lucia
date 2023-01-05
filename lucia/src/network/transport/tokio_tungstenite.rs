use crate::{
  misc::manage_before_sending_related,
  network::{
    transport::{BiTransport, Transport, TransportParams},
    TransportGroup, WebSocket, WsParams, WsReqParamsTy,
  },
  pkg::{Package, PkgsAux},
};
use alloc::string::String;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tungstenite::Message;

/// Shortcut of `WebSocketStream<MaybeTlsStream<TcpStream>>`.
pub type TokioTungstenite = WebSocketStream<MaybeTlsStream<TcpStream>>;

/// ```rust,no_run
/// # async fn fun() -> lucia::Result<()> {
/// use lucia::{
///   network::{transport::Transport, WsParams},
///   pkg::PkgsAux,
/// };
/// let _ = tokio_tungstenite::connect_async("URL")
///   .await?
///   .0
///   .send_retrieve_and_decode_contained(
///     &mut (),
///     &mut PkgsAux::from_minimum((), (), WsParams::default()),
///   )
///   .await?;
/// # Ok(()) }
/// ```
impl<DRSR> Transport<DRSR> for TokioTungstenite {
  const GROUP: TransportGroup = TransportGroup::WebSocket;
  type Params = WsParams;

  #[inline]
  async fn send<P>(
    &mut self,
    pkg: &mut P,
    pkgs_aux: &mut PkgsAux<P::Api, DRSR, Self::Params>,
  ) -> Result<(), P::Error>
  where
    P: Package<DRSR, WsParams>,
  {
    pkgs_aux.byte_buffer.clear();
    manage_before_sending_related(pkg, pkgs_aux, self).await?;
    let vec = pkgs_aux.byte_buffer.clone();
    let msg = match pkgs_aux.tp.ext_req_params_mut().ty {
      WsReqParamsTy::Bytes => Message::Binary(vec),
      WsReqParamsTy::String => Message::Text(String::from_utf8(vec).map_err(Into::into)?),
    };
    <Self as SinkExt<_>>::send(self, msg).await.map_err(Into::into)?;
    pkgs_aux.byte_buffer.clear();
    pkg.after_sending(&mut pkgs_aux.api, pkgs_aux.tp.ext_res_params_mut()).await?;
    pkgs_aux.tp.reset();
    Ok(())
  }

  #[inline]
  async fn send_and_retrieve<P>(
    &mut self,
    pkg: &mut P,
    pkgs_aux: &mut PkgsAux<P::Api, DRSR, Self::Params>,
  ) -> Result<usize, P::Error>
  where
    P: Package<DRSR, WsParams>,
  {
    <Self as Transport<DRSR>>::send(self, pkg, pkgs_aux).await?;
    Ok(if let Some(elem) = self.next().await {
      pkgs_aux.byte_buffer.extend(elem.map_err(Into::into)?.into_data());
      pkgs_aux.byte_buffer.len()
    } else {
      0
    })
  }
}

impl<DRSR> BiTransport<DRSR> for TokioTungstenite {
  #[inline]
  async fn retrieve<API>(
    &mut self,
    pkgs_aux: &mut PkgsAux<API, DRSR, Self::Params>,
  ) -> crate::Result<usize> {
    if let Some(rslt) = self.next().await {
      let data = rslt?.into_data();
      let len = data.len();
      pkgs_aux.byte_buffer.extend(data);
      Ok(len)
    } else {
      Ok(0)
    }
  }
}

impl WebSocket for TokioTungstenite {
  #[inline]
  async fn from_url(url: &str) -> crate::Result<Self> {
    Ok(connect_async(url).await?.0)
  }
}
