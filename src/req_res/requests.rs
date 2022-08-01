use crate::{
  data_format::{JsonRpcResponse, ProcessedJsonRpcResponse},
  dnsn::Deserialize,
  types::Id,
  utils::Buffer,
  Request, RequestParamsModifier,
};
use alloc::vec::Vec;
use arrayvec::ArrayVec;
use core::borrow::Borrow;

/// For data formats that allow and process multiple requests in a single call like JSON-RPC.
pub trait Requests<BU, CP, DRSR, REQP, RESP>: RequestParamsModifier<CP, REQP> {
  /// Process bytes received from a counterpart and stores them in the passed `buffer`.
  ///
  /// `buffer` is always extended with new elements.
  fn manage_responses(
    &self,
    buffer: &mut BU,
    bytes: &[u8],
    drsr: &mut DRSR,
    resp: &RESP,
  ) -> crate::Result<()>;
}

impl<BU, CP, DRSR, REQP, RESP> Requests<BU, CP, DRSR, REQP, RESP> for () {
  #[inline]
  fn manage_responses(&self, _: &mut BU, _: &[u8], _: &mut DRSR, _: &RESP) -> crate::Result<()> {
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
  ) -> crate::Result<()> {
    U::manage_responses(self, buffer, bytes, drsr, resp)
  }
}

impl<BU, CP, DRSR, PR, REQ, REQP, RESP, RR, const N: usize> Requests<BU, CP, DRSR, REQP, RESP>
  for [REQ; N]
where
  BU: Buffer<ProcessedJsonRpcResponse<PR>>,
  REQ: Borrow<Id>
    + PartialOrd
    + Request<
      CP,
      REQP,
      RESP,
      ProcessedResponse = ProcessedJsonRpcResponse<PR>,
      RawResponse = JsonRpcResponse<RR>,
    >,
  REQ::RawResponse: for<'de> Deserialize<'de, DRSR>,
{
  #[inline]
  fn manage_responses(
    &self,
    buffer: &mut BU,
    bytes: &[u8],
    drsr: &mut DRSR,
    resp: &RESP,
  ) -> crate::Result<()> {
    push_responses_from_reqs_slice(buffer, bytes, drsr, resp, &self[..])
  }
}
impl<CP, REQ, REQP, const N: usize> RequestParamsModifier<CP, REQP> for [REQ; N]
where
  REQ: RequestParamsModifier<CP, REQP>,
{
  #[inline]
  fn modify_all_params(cp: &mut CP, reqp: REQP) -> crate::Result<()> {
    REQ::modify_all_params(cp, reqp)
  }
}

impl<BU, CP, DRSR, PR, REQ, REQP, RESP, RR, const N: usize> Requests<BU, CP, DRSR, REQP, RESP>
  for ArrayVec<REQ, N>
where
  BU: Buffer<ProcessedJsonRpcResponse<PR>>,
  REQ: Borrow<Id>
    + PartialOrd
    + Request<
      CP,
      REQP,
      RESP,
      ProcessedResponse = ProcessedJsonRpcResponse<PR>,
      RawResponse = JsonRpcResponse<RR>,
    >,
  REQ::RawResponse: for<'de> Deserialize<'de, DRSR>,
{
  #[inline]
  fn manage_responses(
    &self,
    buffer: &mut BU,
    bytes: &[u8],
    drsr: &mut DRSR,
    resp: &RESP,
  ) -> crate::Result<()> {
    push_responses_from_reqs_slice(buffer, bytes, drsr, resp, self)
  }
}
impl<CP, REQ, REQP, const N: usize> RequestParamsModifier<CP, REQP> for ArrayVec<REQ, N>
where
  REQ: RequestParamsModifier<CP, REQP>,
{
  #[inline]
  fn modify_all_params(params: &mut CP, reqp: REQP) -> crate::Result<()> {
    REQ::modify_all_params(params, reqp)
  }
}

impl<BU, CP, DRSR, PR, REQ, REQP, RR, RESP> Requests<BU, CP, DRSR, REQP, RESP> for [REQ]
where
  BU: Buffer<ProcessedJsonRpcResponse<PR>>,
  REQ: Borrow<Id>
    + PartialOrd
    + Request<
      CP,
      REQP,
      RESP,
      ProcessedResponse = ProcessedJsonRpcResponse<PR>,
      RawResponse = JsonRpcResponse<RR>,
    >,
  REQ::RawResponse: for<'de> Deserialize<'de, DRSR>,
{
  #[inline]
  fn manage_responses(
    &self,
    buffer: &mut BU,
    bytes: &[u8],
    drsr: &mut DRSR,
    resp: &RESP,
  ) -> crate::Result<()> {
    push_responses_from_reqs_slice(buffer, bytes, drsr, resp, self)
  }
}
impl<CP, REQ, REQP> RequestParamsModifier<CP, REQP> for [REQ]
where
  REQ: RequestParamsModifier<CP, REQP>,
{
  #[inline]
  fn modify_all_params(cp: &mut CP, reqp: REQP) -> crate::Result<()> {
    REQ::modify_all_params(cp, reqp)
  }
}

impl<BU, CP, DRSR, PR, REQ, REQP, RR, RESP> Requests<BU, CP, DRSR, REQP, RESP> for &'_ [REQ]
where
  BU: Buffer<ProcessedJsonRpcResponse<PR>>,
  REQ: Borrow<Id>
    + PartialOrd
    + Request<
      CP,
      REQP,
      RESP,
      ProcessedResponse = ProcessedJsonRpcResponse<PR>,
      RawResponse = JsonRpcResponse<RR>,
    >,
  REQ::RawResponse: for<'de> Deserialize<'de, DRSR>,
{
  #[inline]
  fn manage_responses(
    &self,
    buffer: &mut BU,
    bytes: &[u8],
    drsr: &mut DRSR,
    resp: &RESP,
  ) -> crate::Result<()> {
    push_responses_from_reqs_slice(buffer, bytes, drsr, resp, self)
  }
}
impl<CP, REQ, REQP> RequestParamsModifier<CP, REQP> for &'_ [REQ]
where
  REQ: RequestParamsModifier<CP, REQP>,
{
  #[inline]
  fn modify_all_params(cp: &mut CP, reqp: REQP) -> crate::Result<()> {
    REQ::modify_all_params(cp, reqp)
  }
}

impl<BU, CP, DRSR, PR, REQ, REQP, RR, RESP> Requests<BU, CP, DRSR, REQP, RESP> for Vec<REQ>
where
  BU: Buffer<ProcessedJsonRpcResponse<PR>>,

  REQ: Borrow<Id>
    + PartialOrd
    + Request<
      CP,
      REQP,
      RESP,
      ProcessedResponse = ProcessedJsonRpcResponse<PR>,
      RawResponse = JsonRpcResponse<RR>,
    >,
  REQ::RawResponse: for<'de> Deserialize<'de, DRSR>,
{
  #[inline]
  fn manage_responses(
    &self,
    buffer: &mut BU,
    bytes: &[u8],
    drsr: &mut DRSR,
    resp: &RESP,
  ) -> crate::Result<()> {
    push_responses_from_reqs_slice(buffer, bytes, drsr, resp, self)
  }
}
impl<CP, REQ, REQP> RequestParamsModifier<CP, REQP> for Vec<REQ>
where
  REQ: RequestParamsModifier<CP, REQP>,
{
  #[inline]
  fn modify_all_params(cp: &mut CP, reqp: REQP) -> crate::Result<()> {
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
pub(crate) fn manage_json_rpc_response<REQ, RR>(
  req: &REQ,
  res: &JsonRpcResponse<RR>,
) -> crate::Result<()>
where
  REQ: Borrow<Id>,
{
  let id = *req.borrow();
  if id != res.id {
    return Err(crate::Error::JsonRpcSentIdDiffersFromReceivedId(id, res.id));
  }
  Ok(())
}

#[inline]
fn push_responses_from_reqs_slice<BU, CP, DRSR, PR, REQ, REQP, RR, RESP>(
  buffer: &mut BU,
  bytes: &[u8],
  drsr: &mut DRSR,
  resp: &RESP,
  slice: &[REQ],
) -> crate::Result<()>
where
  BU: Buffer<ProcessedJsonRpcResponse<PR>>,
  REQ: Borrow<Id>
    + PartialOrd
    + Request<
      CP,
      REQP,
      RESP,
      ProcessedResponse = ProcessedJsonRpcResponse<PR>,
      RawResponse = JsonRpcResponse<RR>,
    >,
  REQ::RawResponse: for<'de> Deserialize<'de, DRSR>,
{
  if slice.is_empty() {
    return Ok(());
  }
  check_sorted(slice.iter())?;
  let mut idx = 0;
  let mut responses_are_not_sorted = false;
  REQ::RawResponse::seq_from_bytes(bytes, drsr, |res: JsonRpcResponse<RR>| {
    let (req, req_idx) = search_slice(idx, res.id, slice)?;
    if idx != req_idx {
      responses_are_not_sorted = true;
    }
    manage_json_rpc_response(req.borrow(), &res)?;
    buffer.extend([REQ::process(res, resp)?])?;
    idx = idx.wrapping_add(1);
    crate::Result::Ok(())
  })?;
  if responses_are_not_sorted {
    buffer.as_mut().sort_unstable();
  }
  Ok(())
}

// First try indexing and then falls back to binary search
fn search_slice<CP, REQ, REQP, RESP>(
  idx: usize,
  res_id: Id,
  reqs: &[REQ],
) -> crate::Result<(&REQ, usize)>
where
  REQ: Borrow<usize> + Request<CP, REQP, RESP>,
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
