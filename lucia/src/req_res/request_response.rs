use crate::misc::FromErrorTy;

/// Data or payload as well as the expected return data format.
///
/// Request parameters are managed by [crate::req_res::RequestParamsModifier].
///
/// # Types
///
/// * `RESP`: **RES**ponse **P**arameters
pub trait RequestResponse<RESP>: FromErrorTy {
  /// The final data format because sometimes the returned raw data format is not what
  /// is desired for actual usage.
  type ProcessedResponse;
  /// The expected raw data format returned by the counterpart for this request.
  type RawResponse;

  /// Fallible custom-logic that will transform [Self::RawResponse] into [Self::ProcessedResponse].
  fn process(raw: Self::RawResponse, resp: &RESP) -> Result<Self::ProcessedResponse, Self::Error>;
}

impl<T, RESP> RequestResponse<RESP> for &'_ T
where
  T: RequestResponse<RESP>,
{
  type ProcessedResponse = T::ProcessedResponse;
  type RawResponse = T::RawResponse;

  #[inline]
  fn process(raw: Self::RawResponse, resp: &RESP) -> Result<Self::ProcessedResponse, Self::Error> {
    T::process(raw, resp)
  }
}

impl<RESP> RequestResponse<RESP> for () {
  type ProcessedResponse = ();
  type RawResponse = ();

  #[inline]
  fn process(raw: Self::RawResponse, _: &RESP) -> Result<Self::ProcessedResponse, Self::Error> {
    Ok(raw)
  }
}
