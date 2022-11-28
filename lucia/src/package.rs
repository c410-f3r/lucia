//! Groups all elements that interact with packages.

mod batch_package;
mod package_with_helper;
mod packages_aux;

use crate::{
  dnsn::{Deserialize, Serialize},
  network::transport::TransportParams,
};
pub use batch_package::*;
use core::fmt::{Debug, Display};
pub use package_with_helper::*;
pub use packages_aux::*;

/// Groups all necessary information to define requests and responses as well as any desired
/// custom parameter to perform modifications before or after sending.
///
/// # Types
///
/// `DRSR`: DeserializeR/SerializeR
/// `TP`: Transport Parameters
pub trait Package<DRSR, TP>
where
  TP: TransportParams,
{
  /// Which API this package is attached to.
  type Api: Send + Sync;
  /// Any error structure that has the required bounds.
  type Error: Debug + Display + From<crate::Error>;
  /// The expected data format that is going to be sent to an external actor.
  type ExternalRequestContent: Serialize<DRSR>;
  /// The expected data format returned by an external actor.
  type ExternalResponseContent: Deserialize<DRSR>;
  /// Any additional parameters used by this package.
  type PackageParams;

  /// Fallible hook that is automatically called after sending the request.
  #[inline]
  fn after_sending(
    &mut self,
    _api: &mut Self::Api,
    _ext_res_params: &mut TP::ExternalResponseParams,
  ) -> Result<(), Self::Error> {
    Ok(())
  }

  /// Fallible hook that is automatically called before sending the request.
  #[inline]
  fn before_sending(
    &mut self,
    _api: &mut Self::Api,
    _ext_req_params: &mut TP::ExternalRequestParams,
  ) -> Result<(), Self::Error> {
    Ok(())
  }

  /// EXTernal REQuest ConTeNT
  ///
  /// Instance value of the defined [Self::ExternalRequestContent].
  fn ext_req_ctnt(&self) -> &Self::ExternalRequestContent;

  /// Similar to [Self::ext_req_ctnt] but returns a mutable reference instead.
  fn ext_req_ctnt_mut(&mut self) -> &mut Self::ExternalRequestContent;

  /// PacKaGe PARAmeterS
  ///
  /// Instance value of the defined [Self::ExternalRequestContent].
  fn pkg_params(&self) -> &Self::PackageParams;

  /// Similar to [Self::pkg_params] but returns a mutable reference instead.
  fn pkg_params_mut(&mut self) -> &mut Self::PackageParams;
}

impl<DRSR, TP> Package<DRSR, TP> for ()
where
  TP: TransportParams,
{
  type Api = ();
  type Error = crate::Error;
  type ExternalRequestContent = ();
  type ExternalResponseContent = ();
  type PackageParams = ();

  #[inline]
  fn after_sending(
    &mut self,
    _: &mut Self::Api,
    _: &mut TP::ExternalResponseParams,
  ) -> Result<(), Self::Error> {
    Ok(())
  }

  #[inline]
  fn before_sending(
    &mut self,
    _: &mut Self::Api,
    _: &mut TP::ExternalRequestParams,
  ) -> Result<(), Self::Error> {
    Ok(())
  }

  #[inline]
  fn ext_req_ctnt(&self) -> &Self::ExternalRequestContent {
    &()
  }

  #[inline]
  fn ext_req_ctnt_mut(&mut self) -> &mut Self::ExternalRequestContent {
    self
  }

  #[inline]
  fn pkg_params(&self) -> &Self::PackageParams {
    self
  }

  #[inline]
  fn pkg_params_mut(&mut self) -> &mut Self::PackageParams {
    self
  }
}

impl<DRSR, P, TP> Package<DRSR, TP> for &mut P
where
  P: Package<DRSR, TP>,
  TP: TransportParams,
{
  type Api = P::Api;
  type Error = P::Error;
  type ExternalRequestContent = P::ExternalRequestContent;
  type ExternalResponseContent = P::ExternalResponseContent;
  type PackageParams = P::PackageParams;

  #[inline]
  fn after_sending(
    &mut self,
    api: &mut Self::Api,
    ext_res_params: &mut TP::ExternalResponseParams,
  ) -> Result<(), Self::Error> {
    (**self).after_sending(api, ext_res_params)
  }

  #[inline]
  fn before_sending(
    &mut self,
    api: &mut Self::Api,
    ext_req_params: &mut TP::ExternalRequestParams,
  ) -> Result<(), Self::Error> {
    (**self).before_sending(api, ext_req_params)
  }

  #[inline]
  fn ext_req_ctnt(&self) -> &Self::ExternalRequestContent {
    (**self).ext_req_ctnt()
  }

  #[inline]
  fn ext_req_ctnt_mut(&mut self) -> &mut Self::ExternalRequestContent {
    (**self).ext_req_ctnt_mut()
  }

  #[inline]
  fn pkg_params(&self) -> &Self::PackageParams {
    (**self).pkg_params()
  }

  #[inline]
  fn pkg_params_mut(&mut self) -> &mut Self::PackageParams {
    (**self).pkg_params_mut()
  }
}
