use core::fmt::Debug;

/// Additional information or metadata received or transmitted by a transport.
pub trait TransportParams {
  /// For example, HTTP has request headers.
  type ExternalRequestParams: Debug + Send + Sync;
  /// For example, HTTP has response headers.
  type ExternalResponseParams: Debug + Send + Sync;

  /// Used internally to construct `PkgsAux`.
  fn into_parts(self) -> (Self::ExternalRequestParams, Self::ExternalResponseParams);
}

impl TransportParams for () {
  type ExternalRequestParams = ();
  type ExternalResponseParams = ();

  #[inline]
  fn into_parts(self) -> (Self::ExternalRequestParams, Self::ExternalResponseParams) {
    ((), ())
  }
}
