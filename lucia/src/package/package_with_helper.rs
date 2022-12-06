use crate::{
  data_format::JsonRpcRequest, network::transport::TransportParams, package::Package, Id,
};
#[cfg(not(feature = "async-fn-in-trait"))]
use alloc::boxed::Box;
use core::{
  borrow::Borrow,
  cmp::{Ord, Ordering},
  hash::{Hash, Hasher},
};

/// Used to store any type of helper data along side a package.
///
/// # Types
///
/// * `H`: Helper
/// * `P`: Package
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Debug)]
pub struct PackageWithHelper<H, P> {
  /// Helper
  #[cfg_attr(feature = "serde", serde(skip))]
  pub helper: H,
  /// Package
  pub package: P,
}

impl<H, P> PackageWithHelper<H, P> {
  /// Constructor shortcut
  #[inline]
  pub fn new(aux: H, package: P) -> Self {
    Self { helper: aux, package }
  }
}

#[cfg_attr(not(feature = "async-fn-in-trait"), async_trait::async_trait)]
impl<DRSR, H, P, TP> Package<DRSR, TP> for PackageWithHelper<H, P>
where
  H: Send + Sync,
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
    self.package.after_sending(api, ext_res_params).await
  }

  #[inline]
  async fn before_sending(
    &mut self,
    api: &mut Self::Api,
    ext_req_params: &mut TP::ExternalRequestParams,
    req_bytes: &[u8],
  ) -> Result<(), Self::Error> {
    self.package.before_sending(api, ext_req_params, req_bytes).await
  }

  #[inline]
  fn ext_req_content(&self) -> &Self::ExternalRequestContent {
    self.package.ext_req_content()
  }

  #[inline]
  fn ext_req_content_mut(&mut self) -> &mut Self::ExternalRequestContent {
    self.package.ext_req_content_mut()
  }

  #[inline]
  fn pkg_params(&self) -> &Self::PackageParams {
    self.package.pkg_params()
  }

  #[inline]
  fn pkg_params_mut(&mut self) -> &mut Self::PackageParams {
    self.package.pkg_params_mut()
  }
}

impl<H, RP> Borrow<Id> for PackageWithHelper<H, JsonRpcRequest<RP>> {
  #[inline]
  fn borrow(&self) -> &Id {
    &self.package.id
  }
}

impl<H, P> Eq for PackageWithHelper<H, P> where P: Eq {}

impl<H, P> Hash for PackageWithHelper<H, P>
where
  P: Hash,
{
  #[inline]
  fn hash<HA>(&self, state: &mut HA)
  where
    HA: Hasher,
  {
    self.package.hash(state);
  }
}

impl<H, P> Ord for PackageWithHelper<H, P>
where
  P: Ord,
{
  #[inline]
  fn cmp(&self, other: &Self) -> Ordering {
    self.package.cmp(&other.package)
  }
}

impl<H, P> PartialEq for PackageWithHelper<H, P>
where
  P: PartialEq,
{
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.package == other.package
  }
}

impl<H, P> PartialOrd for PackageWithHelper<H, P>
where
  P: PartialOrd,
{
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.package.partial_cmp(&other.package)
  }
}
