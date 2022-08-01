/// Data or payload as well as the expected return data format.
///
/// Request parameters are managed by [crate::RequestParamsModifier].
///
/// # Types
///
/// * `RESP`: **RES**ponse **P**arameters
pub trait RequestResponse<RESP> {
  /// The final data format because sometimes the returned raw data format is not what
  /// is desired for actual usage.
  type ProcessedResponse;
  /// The expected raw data format returned by the counterpart for this request.
  type RawResponse;

  /// Fallible custom-logic that will transform [Self::RawResponse] into [Self::ProcessedResponse].
  fn process(raw: Self::RawResponse, resp: &RESP) -> crate::Result<Self::ProcessedResponse>;
}

impl<T, RESP> RequestResponse<RESP> for &'_ T
where
  T: RequestResponse<RESP>,
{
  type ProcessedResponse = T::ProcessedResponse;
  type RawResponse = T::RawResponse;

  #[inline]
  fn process(raw: Self::RawResponse, resp: &RESP) -> crate::Result<Self::ProcessedResponse> {
    T::process(raw, resp)
  }
}

impl<RESP> RequestResponse<RESP> for () {
  type ProcessedResponse = ();
  type RawResponse = ();

  #[inline]
  fn process(raw: Self::RawResponse, _: &RESP) -> crate::Result<Self::ProcessedResponse> {
    Ok(raw)
  }
}
