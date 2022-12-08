use crate::{
  misc::manage_before_sending_related,
  network::{transport::Transport, TransportGroup, WsParams, WsReqParamsTy},
  package::{Package, PackagesAux},
};
#[cfg(not(feature = "async-fn-in-trait"))]
use alloc::boxed::Box;
use alloc::string::String;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tungstenite::Message;

/// Shortcut of `WebSocketStream<MaybeTlsStream<TcpStream>>`.
pub type TokioTungstenite = WebSocketStream<MaybeTlsStream<TcpStream>>;

/// Handy constructor to avoid having to explicitly import `tokio_tungstenite` dependencies.
///
/// ```rust,no_run
/// # async fn fun() -> lucia::Result<()> {
/// use lucia::{
///   network::{transport::Transport, WsParams},
///   package::PackagesAux,
/// };
/// let _ = tokio_tungstenite::connect_async("URL")
///   .await?
///   .0
///   .send_retrieve_and_decode_contained(
///     &mut (),
///     &mut PackagesAux::from_minimum((), (), WsParams::default()),
///   )
///   .await?;
/// # Ok(()) }
/// ```
#[cfg_attr(not(feature = "async-fn-in-trait"), async_trait::async_trait)]
impl<DRSR> Transport<DRSR> for TokioTungstenite
where
  DRSR: Send + Sync,
{
  const GROUP: TransportGroup = TransportGroup::WebSocket;
  type Params = WsParams;

  #[inline]
  async fn send<P>(
    &mut self,
    pkg: &mut P,
    pkgs_aux: &mut PackagesAux<P::Api, DRSR, Self::Params>,
  ) -> Result<(), P::Error>
  where
    P: Package<DRSR, WsParams> + Send + Sync,
  {
    pkgs_aux.byte_buffer.clear();
    manage_before_sending_related(pkg, pkgs_aux, self).await?;
    let vec = pkgs_aux.byte_buffer.clone();
    let msg = match pkgs_aux.ext_req_params.ty {
      WsReqParamsTy::Bytes => Message::Binary(vec),
      WsReqParamsTy::String => Message::Text(String::from_utf8(vec).map_err(Into::into)?),
    };
    <Self as SinkExt<_>>::send(self, msg).await.map_err(Into::into)?;
    pkgs_aux.byte_buffer.clear();
    pkg.after_sending(&mut pkgs_aux.api, &mut pkgs_aux.ext_res_params).await?;
    pkgs_aux.ext_req_params.clear();
    Ok(())
  }

  #[inline]
  async fn send_and_retrieve<P>(
    &mut self,
    pkg: &mut P,
    pkgs_aux: &mut PackagesAux<P::Api, DRSR, Self::Params>,
  ) -> Result<usize, P::Error>
  where
    P: Package<DRSR, WsParams> + Send + Sync,
  {
    let _res = <Self as Transport<DRSR>>::send(self, pkg, pkgs_aux).await?;
    Ok(if let Some(elem) = self.next().await {
      pkgs_aux.byte_buffer.extend(elem.map_err(Into::into)?.into_data());
      pkgs_aux.byte_buffer.len()
    } else {
      0
    })
  }
}
