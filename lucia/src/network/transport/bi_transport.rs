use crate::{
  dnsn::Deserialize,
  misc::log_res,
  network::transport::Transport,
  pkg::{Package, PkgsAux},
};

/// Bidirectional Transport
///
/// Similar to [Transport] but expects an connection where clients call poll data from the server.
///
/// # Types
///
/// * `DRSR`: `D`eserialize`R`/`S`erialize`R`
pub trait BiTransport<DRSR>: Transport<DRSR> {
  /// Retrieves data from the server filling the internal buffer and returning the amount of
  /// bytes written.
  async fn retrieve<API>(
    &mut self,
    pkgs_aux: &mut PkgsAux<API, DRSR, Self::Params>,
  ) -> crate::Result<usize>;

  /// Internally calls [Self::retrieve] and then tries to decode the defined response specified
  /// in [Package::ExternalResponseContent].
  #[inline]
  async fn retrieve_and_decode_contained<P>(
    &mut self,
    pkgs_aux: &mut PkgsAux<P::Api, DRSR, Self::Params>,
  ) -> Result<P::ExternalResponseContent, P::Error>
  where
    P: Package<DRSR, Self::Params>,
  {
    let len = self.retrieve(pkgs_aux).await?;
    log_res(pkgs_aux.byte_buffer.as_ref());
    let rslt = P::ExternalResponseContent::from_bytes(
      pkgs_aux.byte_buffer.get(..len).unwrap_or_default(),
      &mut pkgs_aux.drsr,
    )?;
    pkgs_aux.byte_buffer.clear();
    Ok(rslt)
  }
}

impl<DRSR, T> BiTransport<DRSR> for &mut T
where
  T: BiTransport<DRSR>,
{
  #[inline]
  async fn retrieve<API>(
    &mut self,
    pkgs_aux: &mut PkgsAux<API, DRSR, Self::Params>,
  ) -> crate::Result<usize> {
    (**self).retrieve(pkgs_aux).await
  }
}
