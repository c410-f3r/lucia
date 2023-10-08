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
  rng::Rng,
  web_socket::{
    compression::NegotiatedCompression, FrameBufferVec, FrameBufferVecMut, FrameMutVec, OpCode,
    WebSocketClient,
  },
  PartitionedBuffer, Stream,
};

#[cfg_attr(feature = "async-trait", async_trait::async_trait)]
impl<DRSR, NC, PB, RNG, S> Transport<DRSR> for (FrameBufferVec, WebSocketClient<NC, PB, RNG, S>)
where
  DRSR: AsyncTrait,
  NC: AsyncTrait + NegotiatedCompression,
  PB: AsyncTrait + BorrowMut<PartitionedBuffer>,
  RNG: AsyncTrait + Rng,
  S: AsyncTrait + Stream,
  for<'read> S::Read<'read>: AsyncTrait,
  for<'write> S::Write<'write>: AsyncTrait,
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
    (&mut self.0, &mut self.1).send(pkg, pkgs_aux).await
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
    (&mut self.0, &mut self.1).send_and_retrieve(pkg, pkgs_aux).await
  }
}

#[cfg_attr(feature = "async-trait", async_trait::async_trait)]
impl<DRSR, NC, PB, RNG, S> BiTransport<DRSR> for (FrameBufferVec, WebSocketClient<NC, PB, RNG, S>)
where
  DRSR: AsyncTrait,
  NC: AsyncTrait + NegotiatedCompression,
  PB: AsyncTrait + BorrowMut<PartitionedBuffer>,
  RNG: AsyncTrait + Rng,
  S: AsyncTrait + Stream,
  for<'read> S::Read<'read>: AsyncTrait,
  for<'write> S::Write<'write>: AsyncTrait,
{
  #[inline]
  async fn retrieve<API>(
    &mut self,
    pkgs_aux: &mut PkgsAux<API, DRSR, Self::Params>,
  ) -> crate::Result<Range<usize>>
  where
    API: AsyncTrait,
  {
    (&mut self.0, &mut self.1).retrieve(pkgs_aux).await
  }
}

#[cfg_attr(feature = "async-trait", async_trait::async_trait)]
impl<DRSR, NC, PB, RNG, S> Transport<DRSR>
  for (&mut FrameBufferVec, &mut WebSocketClient<NC, PB, RNG, S>)
where
  DRSR: AsyncTrait,
  NC: AsyncTrait + NegotiatedCompression,
  PB: AsyncTrait + BorrowMut<PartitionedBuffer>,
  RNG: AsyncTrait + Rng,
  S: AsyncTrait + Stream,
  for<'read> S::Read<'read>: AsyncTrait,
  for<'write> S::Write<'write>: AsyncTrait,
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
    let frame_rslt = FrameMutVec::new_fin(self.0, op_code, &pkgs_aux.byte_buffer);
    self.1.write_frame(&mut frame_rslt.map_err(Into::into)?).await.map_err(Into::into)?;
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
    let fb = &mut FrameBufferVecMut::from(&mut pkgs_aux.byte_buffer);
    let frame = self.1.borrow_mut().read_frame(fb).await.map_err(Into::into)?;
    if let OpCode::Close = frame.op_code() {
      return Err(crate::Error::ClosedWsConnection.into());
    }
    let indcs = frame.fb().indcs();
    Ok(indcs.1.into()..indcs.2)
  }
}

#[cfg_attr(feature = "async-trait", async_trait::async_trait)]
impl<DRSR, NC, PB, RNG, S> BiTransport<DRSR>
  for (&mut FrameBufferVec, &mut WebSocketClient<NC, PB, RNG, S>)
where
  DRSR: AsyncTrait,
  NC: AsyncTrait + NegotiatedCompression,
  PB: AsyncTrait + BorrowMut<PartitionedBuffer>,
  RNG: AsyncTrait + Rng,
  S: AsyncTrait + Stream,
  for<'read> S::Read<'read>: AsyncTrait,
  for<'write> S::Write<'write>: AsyncTrait,
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
    let frame = self.1.borrow_mut().read_frame(fb).await?;
    if let OpCode::Close = frame.op_code() {
      return Err(crate::Error::ClosedWsConnection.into());
    }
    let indcs = frame.fb().indcs();
    Ok(indcs.1.into()..indcs.2)
  }
}
