use crate::{
  dnsn::{Deserialize, Serialize},
  req_res::RequestParamsModifier,
};

/// All communication between the parties of all APIs happens through the structures that implement
/// this trait.
///
/// Can have multiple request parameters as well as multiple response parameters.
///
/// # Types
///
/// * `CP`: `C`ommon `P`arameters
/// * `DRSR`: `D`eserialize`R`/`S`erialize`R`
/// * `REQP`: `REQ`uest `P`arameters
/// * `RESP`: `RES`ponse `P`arameters
pub trait Request<CP, DRSR, REQP, RESP>: RequestParamsModifier<CP, REQP> {
  /// Data payload that is going to be sent.
  type Data: Serialize<DRSR>;
  /// The final data format because sometimes the returned raw data format is not what
  /// is desired for actual usage.
  type ProcessedResponse;
  /// The expected raw data format returned by the counterpart for this request.
  type RawResponse: Deserialize<DRSR>;

  /// A request has to have stored the data that is going to be serialized and sent.
  fn data(&self) -> &Self::Data;

  /// Fallible custom-logic that will transform [Self::RawResponse] into [Self::ProcessedResponse].
  fn process(raw: Self::RawResponse, resp: &RESP) -> Result<Self::ProcessedResponse, Self::Error>;
}

impl<CP, DRSR, REQP, RESP, T> Request<CP, DRSR, REQP, RESP> for &'_ T
where
  T: Request<CP, DRSR, REQP, RESP>,
{
  type Data = T::Data;
  type ProcessedResponse = T::ProcessedResponse;
  type RawResponse = T::RawResponse;

  #[inline]
  fn data(&self) -> &Self::Data {
    (*self).data()
  }

  #[inline]
  fn process(raw: Self::RawResponse, resp: &RESP) -> Result<Self::ProcessedResponse, Self::Error> {
    T::process(raw, resp)
  }
}

impl<CP, DRSR, REQP, RESP> Request<CP, DRSR, REQP, RESP> for () {
  type Data = ();
  type ProcessedResponse = ();
  type RawResponse = ();

  #[inline]
  fn data(&self) -> &Self::Data {
    &()
  }

  #[inline]
  fn process(_: Self::RawResponse, _: &RESP) -> Result<Self::ProcessedResponse, Self::Error> {
    Ok(())
  }
}
