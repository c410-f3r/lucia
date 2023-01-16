use crate::{misc::AsyncTrait, network::transport::Transport, pkg::PkgsAux};
#[cfg(feature = "async-trait")]
use alloc::boxed::Box;

/// Bidirectional Transport
///
/// Similar to [Transport] but expects an connection where clients call poll data from the server.
///
/// # Types
///
/// * `DRSR`: `D`eserialize`R`/`S`erialize`R`
#[cfg_attr(feature = "async-trait", async_trait::async_trait)]
pub trait BiTransport<DRSR>: Transport<DRSR> {
  /// Retrieves data from the server filling the internal buffer and returning the amount of
  /// bytes written.
  async fn retrieve<API>(
    &mut self,
    pkgs_aux: &mut PkgsAux<API, DRSR, Self::Params>,
  ) -> crate::Result<usize>
  where
    API: AsyncTrait;
}

#[cfg_attr(feature = "async-trait", async_trait::async_trait)]
impl<DRSR, T> BiTransport<DRSR> for &mut T
where
  DRSR: AsyncTrait,
  T: BiTransport<DRSR>,
{
  #[inline]
  async fn retrieve<API>(
    &mut self,
    pkgs_aux: &mut PkgsAux<API, DRSR, Self::Params>,
  ) -> crate::Result<usize>
  where
    API: AsyncTrait,
  {
    (**self).retrieve(pkgs_aux).await
  }
}
