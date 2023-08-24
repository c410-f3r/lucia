use crate::{
  dnsn::Deserialize,
  misc::{log_res, AsyncTrait},
  network::transport::Transport,
  pkg::{Package, PkgsAux},
};
#[cfg(feature = "async-trait")]
use alloc::boxed::Box;
use core::ops::Range;

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
  ) -> crate::Result<Range<usize>>
  where
    API: AsyncTrait;

  /// Internally calls [Self::retrieve] and then tries to decode the defined response specified
  /// in [Package::ExternalResponseContent].
  #[inline]
  async fn retrieve_and_decode_contained<P>(
    &mut self,
    pkgs_aux: &mut PkgsAux<P::Api, DRSR, Self::Params>,
  ) -> Result<P::ExternalResponseContent, P::Error>
  where
    DRSR: AsyncTrait,
    P: Package<DRSR, Self::Params>,
  {
    let range = self.retrieve(pkgs_aux).await?;
    log_res(pkgs_aux.byte_buffer.as_ref());
    let rslt = P::ExternalResponseContent::from_bytes(
      pkgs_aux.byte_buffer.get(range).unwrap_or_default(),
      &mut pkgs_aux.drsr,
    )?;
    pkgs_aux.byte_buffer.clear();
    Ok(rslt)
  }
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
  ) -> crate::Result<Range<usize>>
  where
    API: AsyncTrait,
  {
    (**self).retrieve(pkgs_aux).await
  }
}
