use core::fmt::Debug;

/// Data or payload as well as the expected return data format.
///
/// Request parameters are managed by [crate::RequestParams].
pub trait RequestData {
  /// The final data format because sometimes the returned raw data format is not what
  /// is desired for actual usage.
  type ProcessedResponse: Debug;
  /// The expected raw data format returned by the counterpart for this request.
  type RawResponse: Debug;

  /// Every request has to have an unique identifier.
  fn id(&self) -> crate::types::Id;

  /// Fallible custom-logic that will transform [Self::RawResponse] into [Self::ProcessedResponse].
  fn process_response(raw: Self::RawResponse) -> crate::Result<Self::ProcessedResponse>;
}

impl<T> RequestData for &'_ T
where
  T: RequestData,
{
  type ProcessedResponse = T::ProcessedResponse;
  type RawResponse = T::RawResponse;

  #[inline]
  fn id(&self) -> crate::types::Id {
    T::id(self)
  }

  #[inline]
  fn process_response(raw: Self::RawResponse) -> crate::Result<Self::ProcessedResponse> {
    T::process_response(raw)
  }
}

impl RequestData for () {
  type ProcessedResponse = ();
  type RawResponse = ();

  #[inline]
  fn id(&self) -> crate::types::Id {
    0
  }

  #[inline]
  fn process_response(raw: Self::RawResponse) -> crate::Result<Self::ProcessedResponse> {
    Ok(raw)
  }
}
