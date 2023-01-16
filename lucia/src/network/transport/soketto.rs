use crate::{
  misc::{manage_before_sending_related, AsyncTrait},
  network::{
    transport::{BiTransport, Transport, TransportParams},
    TransportGroup, WsParams, WsReqParamsTy,
  },
  pkg::{Package, PkgsAux},
};
#[cfg(feature = "async-trait")]
use alloc::boxed::Box;
use core::str;
use futures::{AsyncRead, AsyncWrite};
use soketto::{
  connection::{Receiver, Sender},
  Data, Incoming,
};

#[cfg_attr(feature = "async-trait", async_trait::async_trait)]
impl<DRSR, T> Transport<DRSR> for (Sender<T>, Receiver<T>)
where
  DRSR: AsyncTrait,
  T: AsyncRead + AsyncTrait + AsyncWrite + Unpin,
{
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
    match pkgs_aux.tp.ext_req_params_mut().ty {
      WsReqParamsTy::Bytes => {
        self.0.send_binary_mut(&mut pkgs_aux.byte_buffer).await.map_err(Into::into)?
      }
      WsReqParamsTy::String => self
        .0
        .send_text(str::from_utf8(&pkgs_aux.byte_buffer).unwrap_or_default())
        .await
        .map_err(Into::into)?,
    };
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
    manage_retrieve(self, pkgs_aux).await.map_err(Into::into)
  }
}

#[cfg_attr(feature = "async-trait", async_trait::async_trait)]
impl<DRSR, T> BiTransport<DRSR> for (Sender<T>, Receiver<T>)
where
  DRSR: AsyncTrait,
  T: AsyncRead + AsyncTrait + AsyncWrite + Unpin,
{
  #[inline]
  async fn retrieve<API>(
    &mut self,
    pkgs_aux: &mut PkgsAux<API, DRSR, Self::Params>,
  ) -> crate::Result<usize>
  where
    API: AsyncTrait,
  {
    manage_retrieve(self, pkgs_aux).await
  }
}

async fn manage_retrieve<API, DRSR, T, TP>(
  this: &mut (Sender<T>, Receiver<T>),
  pkgs_aux: &mut PkgsAux<API, DRSR, TP>,
) -> crate::Result<usize>
where
  T: AsyncRead + AsyncWrite + Unpin,
  TP: TransportParams,
{
  let before = pkgs_aux.byte_buffer.len();
  match this.1.receive(&mut pkgs_aux.byte_buffer).await? {
    Incoming::Data(data) => match data {
      Data::Text(_text) => {}
      Data::Binary(_bytes) => {}
    },
    Incoming::Pong(_bytes) => {}
    Incoming::Closed(_) => {}
  }
  let after = pkgs_aux.byte_buffer.len();
  Ok(after.wrapping_sub(before))
}
