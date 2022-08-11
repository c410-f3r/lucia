use crate::{
  dnsn::{Deserialize, Serialize},
  misc::{Buffer, FromErrorTy},
  req_res::{Request, RequestParamsModifier},
  Id,
};
use alloc::vec::Vec;
use arrayvec::ArrayVec;
use core::borrow::Borrow;

/// For data formats that allow and process multiple requests in a single call like JSON-RPC.
pub trait Requests<BU, CP, DRSR, REQP, RESP>: Request<CP, DRSR, REQP, RESP> {
  /// Process bytes received from a counterpart and stores them in the passed `buffer`.
  ///
  /// `buffer` is always extended with new elements.
  fn manage_responses(
    &self,
    buffer: &mut BU,
    bytes: &[u8],
    drsr: &mut DRSR,
    resp: &RESP,
  ) -> Result<(), Self::Error>;
}

impl<BU, CP, DRSR, REQP, RESP> Requests<BU, CP, DRSR, REQP, RESP> for () {
  #[inline]
  fn manage_responses(
    &self,
    _: &mut BU,
    _: &[u8],
    _: &mut DRSR,
    _: &RESP,
  ) -> Result<(), Self::Error> {
    Ok(())
  }
}

impl<BU, CP, DRSR, REQP, RESP, U> Requests<BU, CP, DRSR, REQP, RESP> for &'_ U
where
  U: Requests<BU, CP, DRSR, REQP, RESP>,
{
  #[inline]
  fn manage_responses(
    &self,
    buffer: &mut BU,
    bytes: &[u8],
    drsr: &mut DRSR,
    resp: &RESP,
  ) -> Result<(), Self::Error> {
    U::manage_responses(self, buffer, bytes, drsr, resp)
  }
}

impl<BU, CP, DRSR, REQ, REQP, RESP, const N: usize> Requests<BU, CP, DRSR, REQP, RESP> for [REQ; N]
where
  BU: Buffer<REQ::ProcessedResponse>,
  REQ: Borrow<Id> + PartialOrd + Request<CP, DRSR, REQP, RESP>,
  REQ::ProcessedResponse: Borrow<Id> + Ord,
  Self: Serialize<DRSR>,
{
  #[inline]
  fn manage_responses(
    &self,
    buffer: &mut BU,
    bytes: &[u8],
    drsr: &mut DRSR,
    resp: &RESP,
  ) -> Result<(), Self::Error> {
    push_responses_from_reqs_slice(buffer, bytes, drsr, resp, &self[..])
  }
}
impl<REQ, const N: usize> FromErrorTy for [REQ; N]
where
  REQ: FromErrorTy,
{
  type Error = REQ::Error;
}
impl<CP, DRSR, REQ, REQP, RESP, const N: usize> Request<CP, DRSR, REQP, RESP> for [REQ; N]
where
  REQ: Request<CP, DRSR, REQP, RESP>,
  Self: Serialize<DRSR>,
{
  type Data = Self;
  type ProcessedResponse = REQ::ProcessedResponse;
  type RawResponse = REQ::RawResponse;

  #[inline]
  fn data(&self) -> &Self::Data {
    self
  }

  #[inline]
  fn process(raw: Self::RawResponse, resp: &RESP) -> Result<Self::ProcessedResponse, Self::Error> {
    REQ::process(raw, resp)
  }
}
impl<CP, REQ, REQP, const N: usize> RequestParamsModifier<CP, REQP> for [REQ; N]
where
  REQ: RequestParamsModifier<CP, REQP>,
{
  #[inline]
  fn modify_all_params(cp: &mut CP, reqp: REQP) -> Result<(), Self::Error> {
    REQ::modify_all_params(cp, reqp)
  }
}

impl<BU, CP, DRSR, REQ, REQP, RESP, const N: usize> Requests<BU, CP, DRSR, REQP, RESP>
  for ArrayVec<REQ, N>
where
  BU: Buffer<REQ::ProcessedResponse>,
  REQ: Borrow<Id> + PartialOrd + Request<CP, DRSR, REQP, RESP>,
  REQ::ProcessedResponse: Borrow<Id> + Ord,
  Self: Serialize<DRSR>,
{
  #[inline]
  fn manage_responses(
    &self,
    buffer: &mut BU,
    bytes: &[u8],
    drsr: &mut DRSR,
    resp: &RESP,
  ) -> Result<(), Self::Error> {
    push_responses_from_reqs_slice(buffer, bytes, drsr, resp, self)
  }
}
impl<REQ, const N: usize> FromErrorTy for ArrayVec<REQ, N>
where
  REQ: FromErrorTy,
{
  type Error = REQ::Error;
}
impl<CP, DRSR, REQ, REQP, RESP, const N: usize> Request<CP, DRSR, REQP, RESP> for ArrayVec<REQ, N>
where
  REQ: Request<CP, DRSR, REQP, RESP>,
  Self: Serialize<DRSR>,
{
  type Data = Self;
  type ProcessedResponse = REQ::ProcessedResponse;
  type RawResponse = REQ::RawResponse;

  #[inline]
  fn data(&self) -> &Self::Data {
    self
  }

  #[inline]
  fn process(raw: Self::RawResponse, resp: &RESP) -> Result<Self::ProcessedResponse, Self::Error> {
    REQ::process(raw, resp)
  }
}
impl<CP, REQ, REQP, const N: usize> RequestParamsModifier<CP, REQP> for ArrayVec<REQ, N>
where
  REQ: RequestParamsModifier<CP, REQP>,
{
  #[inline]
  fn modify_all_params(params: &mut CP, reqp: REQP) -> Result<(), Self::Error> {
    REQ::modify_all_params(params, reqp)
  }
}

impl<BU, CP, DRSR, REQ, REQP, RESP> Requests<BU, CP, DRSR, REQP, RESP> for &'_ [REQ]
where
  BU: Buffer<REQ::ProcessedResponse>,
  REQ: Borrow<Id> + PartialOrd + Request<CP, DRSR, REQP, RESP>,
  REQ::ProcessedResponse: Borrow<Id> + Ord,
  Self: Serialize<DRSR>,
{
  #[inline]
  fn manage_responses(
    &self,
    buffer: &mut BU,
    bytes: &[u8],
    drsr: &mut DRSR,
    resp: &RESP,
  ) -> Result<(), Self::Error> {
    push_responses_from_reqs_slice(buffer, bytes, drsr, resp, self)
  }
}
impl<REQ> FromErrorTy for &'_ [REQ]
where
  REQ: FromErrorTy,
{
  type Error = REQ::Error;
}
impl<CP, DRSR, REQ, REQP, RESP> Request<CP, DRSR, REQP, RESP> for &'_ [REQ]
where
  REQ: Request<CP, DRSR, REQP, RESP>,
  Self: Serialize<DRSR>,
{
  type Data = Self;
  type ProcessedResponse = REQ::ProcessedResponse;
  type RawResponse = REQ::RawResponse;

  #[inline]
  fn data(&self) -> &Self::Data {
    self
  }

  #[inline]
  fn process(raw: Self::RawResponse, resp: &RESP) -> Result<Self::ProcessedResponse, Self::Error> {
    REQ::process(raw, resp)
  }
}
impl<CP, REQ, REQP> RequestParamsModifier<CP, REQP> for &'_ [REQ]
where
  REQ: RequestParamsModifier<CP, REQP>,
{
  #[inline]
  fn modify_all_params(cp: &mut CP, reqp: REQP) -> Result<(), Self::Error> {
    REQ::modify_all_params(cp, reqp)
  }
}

impl<BU, CP, DRSR, REQ, REQP, RESP> Requests<BU, CP, DRSR, REQP, RESP> for Vec<REQ>
where
  BU: Buffer<REQ::ProcessedResponse>,
  REQ: Borrow<Id> + PartialOrd + Request<CP, DRSR, REQP, RESP>,
  REQ::ProcessedResponse: Borrow<Id> + Ord,
  Self: Serialize<DRSR>,
{
  #[inline]
  fn manage_responses(
    &self,
    buffer: &mut BU,
    bytes: &[u8],
    drsr: &mut DRSR,
    resp: &RESP,
  ) -> Result<(), Self::Error> {
    push_responses_from_reqs_slice(buffer, bytes, drsr, resp, self)
  }
}
impl<REQ> FromErrorTy for Vec<REQ>
where
  REQ: FromErrorTy,
{
  type Error = REQ::Error;
}
impl<CP, DRSR, REQ, REQP, RESP> Request<CP, DRSR, REQP, RESP> for Vec<REQ>
where
  REQ: Request<CP, DRSR, REQP, RESP>,
  Self: Serialize<DRSR>,
{
  type Data = Self;
  type ProcessedResponse = REQ::ProcessedResponse;
  type RawResponse = REQ::RawResponse;

  #[inline]
  fn data(&self) -> &Self::Data {
    self
  }

  #[inline]
  fn process(raw: Self::RawResponse, resp: &RESP) -> Result<Self::ProcessedResponse, Self::Error> {
    REQ::process(raw, resp)
  }
}
impl<CP, REQ, REQP> RequestParamsModifier<CP, REQP> for Vec<REQ>
where
  REQ: RequestParamsModifier<CP, REQP>,
{
  #[inline]
  fn modify_all_params(cp: &mut CP, reqp: REQP) -> Result<(), Self::Error> {
    REQ::modify_all_params(cp, reqp)
  }
}

#[inline]
fn check_sorted<T>(mut iter: impl Iterator<Item = T>) -> crate::Result<()>
where
  T: PartialOrd,
{
  let mut is_sorted = true;
  let mut previous = if let Some(elem) = iter.next() { elem } else { return Ok(()) };
  for curr in iter {
    if previous > curr {
      is_sorted = false;
      break;
    } else {
      previous = curr;
    }
  }
  if is_sorted {
    Ok(())
  } else {
    Err(crate::Error::JsonRpcRequestsAreNotSorted)
  }
}

#[inline]
pub(crate) fn manage_json_rpc_response<REQ, RR>(req: &REQ, res: &RR) -> crate::Result<()>
where
  REQ: Borrow<Id>,
  RR: Borrow<Id>,
{
  let req_id = *req.borrow();
  let res_id = *res.borrow();
  if req_id != res_id {
    return Err(crate::Error::JsonRpcSentIdDiffersFromReceivedId(req_id, res_id));
  }
  Ok(())
}

#[inline]
fn push_responses_from_reqs_slice<CP, BU, DRSR, REQ, REQP, RESP>(
  buffer: &mut BU,
  bytes: &[u8],
  drsr: &mut DRSR,
  resp: &RESP,
  slice: &[REQ],
) -> Result<(), REQ::Error>
where
  BU: Buffer<REQ::ProcessedResponse>,
  REQ: Borrow<Id> + PartialOrd + Request<CP, DRSR, REQP, RESP>,
  REQ::ProcessedResponse: Borrow<Id> + Ord,
{
  if slice.is_empty() {
    return Ok(());
  }
  check_sorted(slice.iter())?;
  let mut idx = 0;
  let mut responses_are_not_sorted = false;
  REQ::RawResponse::seq_from_bytes(bytes, drsr, |raw: REQ::RawResponse| {
    let processed = REQ::process(raw, resp)?;
    let (req, req_idx) = search_slice(idx, *processed.borrow(), slice)?;
    if idx != req_idx {
      responses_are_not_sorted = true;
    }
    manage_json_rpc_response(req.borrow(), &processed)?;
    buffer.push(processed).map_err(Into::into)?;
    idx = idx.wrapping_add(1);
    Ok::<_, REQ::Error>(())
  })?;
  if responses_are_not_sorted {
    buffer.as_mut().sort_unstable();
  }
  Ok(())
}

// First try indexing and then falls back to binary search
fn search_slice<CP, DRSR, REQ, REQP, RESP>(
  idx: usize,
  res_id: Id,
  reqs: &[REQ],
) -> crate::Result<(&REQ, usize)>
where
  REQ: Borrow<usize> + Request<CP, DRSR, REQP, RESP>,
{
  let opt = reqs.get(idx).and_then(|req| (*req.borrow() == res_id).then(|| req));
  if let Some(elem) = opt {
    Ok((elem, idx))
  } else {
    reqs
      .binary_search_by(|req| req.borrow().cmp(&res_id))
      .ok()
      .and_then(|local_idx| Some((reqs.get(local_idx)?, local_idx)))
      .ok_or(crate::Error::JsonRpcResponseIsNotPresentInAnySentRequest(res_id))
  }
}
