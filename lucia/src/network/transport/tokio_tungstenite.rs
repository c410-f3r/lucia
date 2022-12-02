use crate::{
  misc::manage_before_sending_related,
  network::{transport::Transport, TransportGroup, WsParams},
  package::{Package, PackagesAux},
};
use alloc::boxed::Box;
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
#[async_trait::async_trait]
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
    <Self as SinkExt<_>>::send(self, Message::Binary(pkgs_aux.byte_buffer.clone()))
      .await
      .map_err(Into::into)?;
    pkgs_aux.byte_buffer.clear();
    pkg.after_sending(&mut pkgs_aux.api, &mut pkgs_aux.ext_res_params).await?;
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
