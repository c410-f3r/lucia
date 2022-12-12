//! # Package
//!
//! Groups all elements that interact with packages.

mod batch_pkg;
mod pkg_with_helper;
mod pkgs_aux;

use crate::{
  dnsn::{Deserialize, Serialize},
  network::transport::TransportParams,
  Api,
};
#[cfg(feature = "async-trait")]
use alloc::boxed::Box;
pub use batch_pkg::{BatchElems, BatchPkg};
use core::fmt::Display;
pub use pkg_with_helper::*;
pub use pkgs_aux::*;

/// Groups all necessary information to define requests and responses as well as any desired
/// custom parameter to perform modifications before or after sending.
///
/// # Types
///
/// `DRSR`: DeserializeR/SerializeR
/// `TP`: Transport Parameters
#[cfg_attr(feature = "async-trait", async_trait::async_trait)]
pub trait Package<DRSR, TP>
where
  TP: TransportParams,
{
  /// Which API this package is attached to.
  type Api: Api<Error = Self::Error> + Send + Sync;
  /// Any custom error structure that can be constructed from [crate::Error].
  type Error: Display + From<crate::Error>;
  /// The expected data format that is going to be sent to an external actor.
  type ExternalRequestContent: Serialize<DRSR>;
  /// The expected data format returned by an external actor.
  type ExternalResponseContent: Deserialize<DRSR>;
  /// Any additional parameters used by this package.
  type PackageParams;

  /// Fallible hook that is automatically called after sending the request described in this
  /// package.
  #[inline]
  async fn after_sending(
    &mut self,
    _api: &mut Self::Api,
    _ext_res_params: &mut TP::ExternalResponseParams,
  ) -> Result<(), Self::Error> {
    Ok(())
  }

  /// Fallible hook that is automatically called before sending the request described in this
  /// package.
  #[inline]
  async fn before_sending(
    &mut self,
    _api: &mut Self::Api,
    _ext_req_params: &mut TP::ExternalRequestParams,
    _req_bytes: &[u8],
  ) -> Result<(), Self::Error> {
    Ok(())
  }

  /// External Request Content
  ///
  /// Instance value of the defined [Self::ExternalRequestContent].
  fn ext_req_content(&self) -> &Self::ExternalRequestContent;

  /// Similar to [Self::ext_req_content] but returns a mutable reference instead.
  fn ext_req_content_mut(&mut self) -> &mut Self::ExternalRequestContent;

  /// Package Parameters
  ///
  /// Instance value of the defined [Self::ExternalRequestContent].
  fn pkg_params(&self) -> &Self::PackageParams;

  /// Similar to [Self::pkg_params] but returns a mutable reference instead.
  fn pkg_params_mut(&mut self) -> &mut Self::PackageParams;
}

#[cfg_attr(feature = "async-trait", async_trait::async_trait)]
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
  fn ext_req_content(&self) -> &Self::ExternalRequestContent {
    &()
  }

  #[inline]
  fn ext_req_content_mut(&mut self) -> &mut Self::ExternalRequestContent {
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

#[cfg_attr(feature = "async-trait", async_trait::async_trait)]
impl<DRSR, P, TP> Package<DRSR, TP> for &mut P
where
  P: Package<DRSR, TP> + Send + Sync,
  TP: TransportParams,
{
  type Api = P::Api;
  type Error = P::Error;
  type ExternalRequestContent = P::ExternalRequestContent;
  type ExternalResponseContent = P::ExternalResponseContent;
  type PackageParams = P::PackageParams;

  #[inline]
  async fn after_sending(
    &mut self,
    api: &mut Self::Api,
    ext_res_params: &mut TP::ExternalResponseParams,
  ) -> Result<(), Self::Error> {
    (**self).after_sending(api, ext_res_params).await
  }

  #[inline]
  async fn before_sending(
    &mut self,
    api: &mut Self::Api,
    ext_req_params: &mut TP::ExternalRequestParams,
    req_bytes: &[u8],
  ) -> Result<(), Self::Error> {
    (**self).before_sending(api, ext_req_params, req_bytes).await
  }

  #[inline]
  fn ext_req_content(&self) -> &Self::ExternalRequestContent {
    (**self).ext_req_content()
  }

  #[inline]
  fn ext_req_content_mut(&mut self) -> &mut Self::ExternalRequestContent {
    (**self).ext_req_content_mut()
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
