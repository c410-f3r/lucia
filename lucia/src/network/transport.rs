//! Implementations of the [Transport] trait.

mod mock;
#[cfg(feature = "reqwest")]
mod reqwest;
#[cfg(feature = "std")]
mod std;
#[cfg(feature = "surf")]
mod surf;
#[cfg(feature = "tokio-tungstenite")]
mod tokio_tungstenite;
mod transport_params;
mod unit;

#[cfg(feature = "tokio-tungstenite")]
pub use self::tokio_tungstenite::*;
pub use mock::*;
pub use transport_params::*;

use crate::{
  dnsn::{Deserialize, Serialize},
  misc::log_res,
  network::TransportGroup,
  pkg::{BatchElems, BatchPkg, Package, PkgsAux},
  Id,
};
#[cfg(feature = "async-trait")]
use alloc::boxed::Box;
use cl_aux::DynContigColl;
use core::borrow::Borrow;

/// Any means of transferring data between two parties.
///
/// Please, see the [crate::pkg::Package] implementation of the desired package to know
/// more about the expected types as well as any other additional documentation.
///
/// # Types
///
/// * `DRSR`: `D`eserialize`R`/`S`erialize`R`
#[cfg_attr(feature = "async-trait", async_trait::async_trait)]
pub trait Transport<DRSR> {
  /// Every transport has an [TransportGroup] identifier.
  const GROUP: TransportGroup;
  /// Every transport has request and response parameters.
  type Params: TransportParams;

  /// Sends a request without trying to retrieve any counterpart data.
  async fn send<P>(
    &mut self,
    pkg: &mut P,
    pkgs_aux: &mut PkgsAux<P::Api, DRSR, Self::Params>,
  ) -> Result<(), P::Error>
  where
    P: Package<DRSR, Self::Params> + Send + Sync;

  /// Sends a request and then awaits its counterpart data response.
  ///
  /// The returned bytes are stored in `pkgs_aux` and its length is returned by this method.
  async fn send_and_retrieve<P>(
    &mut self,
    pkg: &mut P,
    pkgs_aux: &mut PkgsAux<P::Api, DRSR, Self::Params>,
  ) -> Result<usize, P::Error>
  where
    P: Package<DRSR, Self::Params> + Send + Sync;

  /// Convenient method similar to [Self::send_retrieve_and_decode_contained] but used for batch
  /// requests
  #[inline]
  async fn send_retrieve_and_decode_batch<'slice, B, P>(
    &mut self,
    buffer: &mut B,
    pkgs: &'slice mut [P],
    pkgs_aux: &mut PkgsAux<P::Api, DRSR, Self::Params>,
  ) -> Result<(), P::Error>
  where
    B: DynContigColl<P::ExternalResponseContent> + Send + Sync,
    DRSR: Send + Sync,
    P: Package<DRSR, Self::Params> + Send + Sync,
    P::ExternalRequestContent: Borrow<Id> + Ord,
    P::ExternalResponseContent: Borrow<Id> + Ord,
    for<'any> BatchElems<'any, DRSR, P, Self::Params>: Serialize<DRSR>,
    Self::Params: Send + Sync,
  {
    let batch_package = &mut BatchPkg::new(pkgs);
    let len = self.send_and_retrieve(batch_package, pkgs_aux).await?;
    log_res(pkgs_aux.byte_buffer.as_ref());
    batch_package.decode_and_push_from_bytes(
      buffer,
      pkgs_aux.byte_buffer.get(..len).unwrap_or_default(),
      &mut pkgs_aux.drsr,
    )?;
    Ok(())
  }

  /// Internally calls [Self::send_and_retrieve] and then tries to decode the defined response specified
  /// in [Package::ExternalResponseContent].
  #[inline]
  async fn send_retrieve_and_decode_contained<P>(
    &mut self,
    pkg: &mut P,
    pkgs_aux: &mut PkgsAux<P::Api, DRSR, Self::Params>,
  ) -> Result<P::ExternalResponseContent, P::Error>
  where
    DRSR: Send + Sync,
    P: Package<DRSR, Self::Params> + Send + Sync,
  {
    let len = self.send_and_retrieve(pkg, pkgs_aux).await?;
    log_res(pkgs_aux.byte_buffer.as_ref());
    Ok(P::ExternalResponseContent::from_bytes(
      pkgs_aux.byte_buffer.get(..len).unwrap_or_default(),
      &mut pkgs_aux.drsr,
    )?)
  }

  /// Instance counterpart of [Self::GROUP].
  #[inline]
  fn ty(&self) -> TransportGroup {
    Self::GROUP
  }
}

#[cfg_attr(feature = "async-trait", async_trait::async_trait)]
impl<DRSR, T> Transport<DRSR> for &mut T
where
  DRSR: Send + Sync,
  T: Send + Sync + Transport<DRSR>,
{
  const GROUP: TransportGroup = T::GROUP;
  type Params = T::Params;

  #[inline]
  async fn send<P>(
    &mut self,
    pkg: &mut P,
    pkgs_aux: &mut PkgsAux<P::Api, DRSR, Self::Params>,
  ) -> Result<(), P::Error>
  where
    P: Package<DRSR, Self::Params> + Send + Sync,
  {
    (**self).send(pkg, pkgs_aux).await
  }

  #[inline]
  async fn send_and_retrieve<P>(
    &mut self,
    pkg: &mut P,
    pkgs_aux: &mut PkgsAux<P::Api, DRSR, Self::Params>,
  ) -> Result<usize, P::Error>
  where
    P: Package<DRSR, Self::Params> + Send + Sync,
  {
    (**self).send_and_retrieve(pkg, pkgs_aux).await
  }
}

#[cfg(test)]
mod tests {
  use crate::{
    dnsn::{Deserialize, Serialize},
    misc::ByteBuffer,
    network::transport::TransportParams,
    pkg::Package,
  };

  #[derive(Debug, Eq, PartialEq)]
  pub(crate) struct PingPong(pub(crate) Ping, pub(crate) ());

  impl<DRSR, TP> Package<DRSR, TP> for PingPong
  where
    DRSR: Send + Sync,
    TP: Send + Sync + TransportParams,
  {
    type Api = ();
    type Error = crate::Error;
    type ExternalRequestContent = Ping;
    type ExternalResponseContent = Pong;
    type PackageParams = ();

    fn ext_req_content(&self) -> &Self::ExternalRequestContent {
      &self.0
    }

    fn ext_req_content_mut(&mut self) -> &mut Self::ExternalRequestContent {
      &mut self.0
    }

    fn pkg_params(&self) -> &Self::PackageParams {
      &self.1
    }

    fn pkg_params_mut(&mut self) -> &mut Self::PackageParams {
      &mut self.1
    }
  }

  #[derive(Debug, Eq, PartialEq)]
  pub(crate) struct Ping;

  impl<DRSR> Serialize<DRSR> for Ping {
    fn to_bytes<BB>(&mut self, bytes: &mut BB, _: &mut DRSR) -> crate::Result<()>
    where
      BB: ByteBuffer,
    {
      bytes.extend(*b"ping")?;
      Ok(())
    }
  }

  #[derive(Debug, Eq, PartialEq)]
  pub(crate) struct Pong(pub(crate) &'static str);

  impl<DRSR> Deserialize<DRSR> for Pong {
    #[inline]
    fn from_bytes(bytes: &[u8], _: &mut DRSR) -> crate::Result<Self> {
      assert_eq!(bytes, b"ping");
      Ok(Self("pong"))
    }

    #[inline]
    fn seq_from_bytes<E>(
      _: &[u8],
      _: &mut DRSR,
      _: impl FnMut(Self) -> Result<(), E>,
    ) -> Result<(), E>
    where
      E: From<crate::Error>,
    {
      Ok(())
    }
  }
}
