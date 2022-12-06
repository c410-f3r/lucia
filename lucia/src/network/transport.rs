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
  package::{BatchElems, BatchPackage, Package, PackagesAux},
  Id,
};
#[cfg(not(feature = "async-fn-in-trait"))]
use alloc::boxed::Box;
use cl_aux::DynContigColl;
use core::borrow::Borrow;

/// Any means of transferring data between two parties.
///
/// Please, see the [crate::package::Package] implementation of the desired package to know
/// more about the expected types as well as any other additional documentation.
///
/// # Types
///
/// * `DRSR`: `D`eserialize`R`/`S`erialize`R`
#[cfg_attr(not(feature = "async-fn-in-trait"), async_trait::async_trait)]
pub trait Transport<DRSR> {
  /// Every transport has an [TransportGroup] identifier.
  const GROUP: TransportGroup;
  /// Every transport has request and response parameters.
  type Params: TransportParams;

  /// Sends a request without trying to retrieve any counterpart data.
  async fn send<P>(
    &mut self,
    pkg: &mut P,
    pkgs_aux: &mut PackagesAux<P::Api, DRSR, Self::Params>,
  ) -> Result<(), P::Error>
  where
    P: Package<DRSR, Self::Params> + Send + Sync;

  /// Sends a request and then awaits its counterpart data response.
  ///
  /// The returned bytes are stored in `pkgs_aux` and its length is returned by this method.
  async fn send_and_retrieve<P>(
    &mut self,
    pkg: &mut P,
    pkgs_aux: &mut PackagesAux<P::Api, DRSR, Self::Params>,
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
    pkgs_aux: &mut PackagesAux<P::Api, DRSR, Self::Params>,
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
    let batch_package = &mut BatchPackage::new(pkgs);
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
    pkgs_aux: &mut PackagesAux<P::Api, DRSR, Self::Params>,
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

#[cfg_attr(not(feature = "async-fn-in-trait"), async_trait::async_trait)]
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
    pkgs_aux: &mut PackagesAux<P::Api, DRSR, Self::Params>,
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
    pkgs_aux: &mut PackagesAux<P::Api, DRSR, Self::Params>,
  ) -> Result<usize, P::Error>
  where
    P: Package<DRSR, Self::Params> + Send + Sync,
  {
    (**self).send_and_retrieve(pkg, pkgs_aux).await
  }
}
