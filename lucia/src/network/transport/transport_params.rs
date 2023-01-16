use core::fmt::Debug;

use crate::misc::AsyncTrait;

/// Additional information or metadata received or transmitted by a transport.
pub trait TransportParams: AsyncTrait {
  /// For example, HTTP has request headers.
  type ExternalRequestParams: AsyncTrait + Debug;
  /// For example, HTTP has response headers.
  type ExternalResponseParams: AsyncTrait + Debug;

  /// External Request Parameters.
  fn ext_req_params(&self) -> &Self::ExternalRequestParams;

  /// Mutable version of [Self::ext_req_params].
  fn ext_req_params_mut(&mut self) -> &mut Self::ExternalRequestParams;

  /// External Response Parameters.
  fn ext_res_params(&self) -> &Self::ExternalResponseParams;

  /// Mutable version of [Self::ext_res_params].
  fn ext_res_params_mut(&mut self) -> &mut Self::ExternalResponseParams;

  /// Sets the inner parameters with their default values.
  fn reset(&mut self);
}

impl TransportParams for () {
  type ExternalRequestParams = ();
  type ExternalResponseParams = ();

  #[inline]
  fn ext_req_params(&self) -> &Self::ExternalRequestParams {
    self
  }

  #[inline]
  fn ext_req_params_mut(&mut self) -> &mut Self::ExternalRequestParams {
    self
  }

  #[inline]
  fn ext_res_params(&self) -> &Self::ExternalResponseParams {
    self
  }

  #[inline]
  fn ext_res_params_mut(&mut self) -> &mut Self::ExternalResponseParams {
    self
  }

  #[inline]
  fn reset(&mut self) {}
}
