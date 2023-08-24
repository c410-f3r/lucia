use crate::{
  misc::{manage_after_sending_related, manage_before_sending_related, AsyncTrait},
  network::{
    transport::{BiTransport, Transport, TransportParams},
    TransportGroup, WsParams, WsReqParamsTy,
  },
  pkg::{Package, PkgsAux},
};
#[cfg(feature = "async-trait")]
use alloc::boxed::Box;
use core::{borrow::BorrowMut, ops::Range};
use wtx::{
  web_socket::{FrameBufferVec, FrameBufferVecMut, FrameMutVec, OpCode, WebSocketClient},
  ReadBuffer, Stream,
};

#[cfg_attr(feature = "async-trait", async_trait::async_trait)]
impl<DRSR, RB, S> Transport<DRSR> for (FrameBufferVec, WebSocketClient<RB, S>)
where
  DRSR: AsyncTrait,
  RB: AsyncTrait + BorrowMut<ReadBuffer>,
  S: AsyncTrait + Stream,
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
    let op_code = match pkgs_aux.tp.ext_req_params_mut().ty {
      WsReqParamsTy::Bytes => OpCode::Binary,
      WsReqParamsTy::String => OpCode::Text,
    };
    let frame = FrameMutVec::new_fin(&mut self.0, op_code, &pkgs_aux.byte_buffer);
    self.1.write_frame(&mut frame.map_err(Into::into)?).await.map_err(Into::into)?;
    pkgs_aux.byte_buffer.clear();
    manage_after_sending_related(pkg, pkgs_aux, self).await?;
    pkgs_aux.tp.reset();
    Ok(())
  }

  #[inline]
  async fn send_and_retrieve<P>(
    &mut self,
    pkg: &mut P,
    pkgs_aux: &mut PkgsAux<P::Api, DRSR, Self::Params>,
  ) -> Result<Range<usize>, P::Error>
  where
    P: Package<DRSR, WsParams>,
  {
    <Self as Transport<DRSR>>::send(self, pkg, pkgs_aux).await?;
    let indcs = self
      .1
      .read_msg(&mut FrameBufferVecMut::from(&mut pkgs_aux.byte_buffer))
      .await
      .map_err(Into::into)?
      .fb()
      .indcs();
    Ok(indcs.1.into()..indcs.2)
  }
}

#[cfg_attr(feature = "async-trait", async_trait::async_trait)]
impl<DRSR, RB, S> BiTransport<DRSR> for (FrameBufferVec, WebSocketClient<RB, S>)
where
  DRSR: AsyncTrait,
  RB: AsyncTrait + BorrowMut<ReadBuffer>,
  S: AsyncTrait + Stream,
{
  #[inline]
  async fn retrieve<API>(
    &mut self,
    pkgs_aux: &mut PkgsAux<API, DRSR, Self::Params>,
  ) -> crate::Result<Range<usize>>
  where
    API: AsyncTrait,
  {
    pkgs_aux.byte_buffer.clear();
    let fb = &mut FrameBufferVecMut::from(&mut pkgs_aux.byte_buffer);
    let indcs = self.1.read_msg(fb).await?.fb().indcs();
    Ok(indcs.1.into()..indcs.2)
  }
}
