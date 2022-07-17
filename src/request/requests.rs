use crate::{
  protocol::{JsonRpcResponse, ProcessedJsonRpcResponse},
  types::Id,
  utils::{manage_json_rpc_response, SeqVisitor},
  Request, RequestParams,
};
use alloc::{collections::BTreeSet, vec::Vec};
use arrayvec::ArrayVec;
use cl_aux::SizeHint;
use core::{borrow::Borrow, fmt::Debug};
use serde::{Deserialize, Deserializer};

/// For protocols that allow and process multiple requests in a single call like JSON-RPC
/// or a normal request that return a collection of items.
pub trait Requests<BU, CP, RPD>: RequestParams<CP, RPD> + SizeHint {
  /// Process bytes received from a counterpart and stores them in the passed `buffer`.
  ///
  /// `buffer` is always extended with new elements.
  fn manage_responses(&self, buffer: &mut BU, bytes: &[u8]) -> crate::Result<()>;
}

impl<BU, CP, RPD> Requests<BU, CP, RPD> for () {
  #[inline]
  fn manage_responses(&self, _: &mut BU, _: &[u8]) -> crate::Result<()> {
    Ok(())
  }
}

impl<BU, CP, RPD, T> Requests<BU, CP, RPD> for &'_ T
where
  T: Requests<BU, CP, RPD>,
{
  #[inline]
  fn manage_responses(&self, buffer: &mut BU, bytes: &[u8]) -> crate::Result<()> {
    T::manage_responses(self, buffer, bytes)
  }
}

impl<BU, CP, PR, REQ, RPD, RR> Requests<BU, CP, RPD> for &'_ [REQ]
where
  BU: AsMut<[ProcessedJsonRpcResponse<PR>]>
    + cl_aux::Extend<ProcessedJsonRpcResponse<PR>, Error = cl_aux::Error, Output = ()>,
  REQ: Debug
    + PartialOrd
    + Request<
      CP,
      RPD,
      ProcessedResponse = ProcessedJsonRpcResponse<PR>,
      RawResponse = JsonRpcResponse<RR>,
    >,
  RR: Debug + for<'de> Deserialize<'de>,
{
  #[inline]
  fn manage_responses(&self, buffer: &mut BU, bytes: &[u8]) -> crate::Result<()> {
    push_responses_from_reqs_slice(buffer, bytes, self)
  }
}
impl<CP, R, RPD> RequestParams<CP, RPD> for &'_ [R]
where
  R: Request<CP, RPD>,
{
  #[inline]
  fn modify_all_params(cp: &mut CP, rpd: RPD) -> crate::Result<()> {
    R::modify_all_params(cp, rpd)
  }
}

impl<BU, CP, PR, REQ, RPD, RR> Requests<BU, CP, RPD> for [REQ]
where
  BU: AsMut<[ProcessedJsonRpcResponse<PR>]>
    + cl_aux::Extend<ProcessedJsonRpcResponse<PR>, Error = cl_aux::Error, Output = ()>,
  REQ: Debug
    + PartialOrd
    + Request<
      CP,
      RPD,
      ProcessedResponse = ProcessedJsonRpcResponse<PR>,
      RawResponse = JsonRpcResponse<RR>,
    >,
  RR: Debug + for<'de> Deserialize<'de>,
{
  #[inline]
  fn manage_responses(&self, buffer: &mut BU, bytes: &[u8]) -> crate::Result<()> {
    push_responses_from_reqs_slice(buffer, bytes, self)
  }
}
impl<CP, R, RPD> RequestParams<CP, RPD> for [R]
where
  R: Request<CP, RPD>,
{
  #[inline]
  fn modify_all_params(cp: &mut CP, rpd: RPD) -> crate::Result<()> {
    R::modify_all_params(cp, rpd)
  }
}

impl<BU, CP, PR, REQ, RPD, RR, const N: usize> Requests<BU, CP, RPD> for [REQ; N]
where
  BU: AsMut<[ProcessedJsonRpcResponse<PR>]>
    + cl_aux::Extend<ProcessedJsonRpcResponse<PR>, Error = cl_aux::Error, Output = ()>,
  REQ: Debug
    + PartialOrd
    + Request<
      CP,
      RPD,
      ProcessedResponse = ProcessedJsonRpcResponse<PR>,
      RawResponse = JsonRpcResponse<RR>,
    >,
  RR: Debug + for<'de> Deserialize<'de>,
{
  #[inline]
  fn manage_responses(&self, buffer: &mut BU, bytes: &[u8]) -> crate::Result<()> {
    push_responses_from_reqs_slice(buffer, bytes, &self[..])
  }
}
impl<CP, R, RPD, const N: usize> RequestParams<CP, RPD> for [R; N]
where
  R: Request<CP, RPD>,
{
  #[inline]
  fn modify_all_params(cp: &mut CP, rpd: RPD) -> crate::Result<()> {
    R::modify_all_params(cp, rpd)
  }
}

impl<BU, CP, PR, REQ, RPD, RR, const N: usize> Requests<BU, CP, RPD> for ArrayVec<REQ, N>
where
  BU: AsMut<[ProcessedJsonRpcResponse<PR>]>
    + cl_aux::Extend<ProcessedJsonRpcResponse<PR>, Error = cl_aux::Error, Output = ()>,
  REQ: Debug
    + PartialOrd
    + Request<
      CP,
      RPD,
      ProcessedResponse = ProcessedJsonRpcResponse<PR>,
      RawResponse = JsonRpcResponse<RR>,
    >,
  RR: Debug + for<'de> Deserialize<'de>,
{
  #[inline]
  fn manage_responses(&self, buffer: &mut BU, bytes: &[u8]) -> crate::Result<()> {
    push_responses_from_reqs_slice(buffer, bytes, self)
  }
}
impl<CP, R, RPD, const N: usize> RequestParams<CP, RPD> for ArrayVec<R, N>
where
  R: Request<CP, RPD>,
{
  #[inline]
  fn modify_all_params(params: &mut CP, rpd: RPD) -> crate::Result<()> {
    R::modify_all_params(params, rpd)
  }
}

impl<BU, CP, PR, REQ, RPD, RR> Requests<BU, CP, RPD> for BTreeSet<REQ>
where
  BU: cl_aux::Extend<ProcessedJsonRpcResponse<PR>, Error = cl_aux::Error, Output = ()>,
  REQ: Borrow<usize>
    + Debug
    + Request<
      CP,
      RPD,
      ProcessedResponse = ProcessedJsonRpcResponse<PR>,
      RawResponse = JsonRpcResponse<RR>,
    > + Ord,
  RR: Debug + for<'de> Deserialize<'de>,
{
  #[inline]
  fn manage_responses(&self, buffer: &mut BU, bytes: &[u8]) -> crate::Result<()> {
    if self.is_empty() {
      return Ok(());
    }
    push_responses_from_reqs_cb::<_, _, _, _, _, REQ, _, _>(buffer, bytes, |id| {
      self.get(&id).ok_or(crate::Error::JsonRpcResponseIsNotPresentInAnySentRequest(id))
    })
  }
}
impl<CP, R, RPD> RequestParams<CP, RPD> for BTreeSet<R>
where
  R: Request<CP, RPD>,
{
  #[inline]
  fn modify_all_params(cp: &mut CP, rpd: RPD) -> crate::Result<()> {
    R::modify_all_params(cp, rpd)
  }
}

#[cfg(feature = "std")]
impl<BU, CP, PR, REQ, RPD, RR> Requests<BU, CP, RPD> for std::collections::HashSet<REQ>
where
  BU: AsMut<[ProcessedJsonRpcResponse<PR>]>
    + cl_aux::Extend<ProcessedJsonRpcResponse<PR>, Error = cl_aux::Error, Output = ()>,
  REQ: Borrow<usize>
    + Debug
    + Eq
    + core::hash::Hash
    + Request<
      CP,
      RPD,
      ProcessedResponse = ProcessedJsonRpcResponse<PR>,
      RawResponse = JsonRpcResponse<RR>,
    >,
  RR: Debug + for<'de> Deserialize<'de>,
{
  #[inline]
  fn manage_responses(&self, buffer: &mut BU, bytes: &[u8]) -> crate::Result<()> {
    if self.is_empty() {
      return Ok(());
    }
    push_responses_from_reqs_cb::<_, _, _, _, _, REQ, _, _>(buffer, bytes, |id| {
      self.get(&id).ok_or(crate::Error::JsonRpcResponseIsNotPresentInAnySentRequest(id))
    })
  }
}
#[cfg(feature = "std")]
impl<CP, RPD, R> RequestParams<CP, RPD> for std::collections::HashSet<R>
where
  R: Request<CP, RPD>,
{
  #[inline]
  fn modify_all_params(cp: &mut CP, rpd: RPD) -> crate::Result<()> {
    R::modify_all_params(cp, rpd)
  }
}

impl<BU, CP, PR, REQ, RPD, RR> Requests<BU, CP, RPD> for Vec<REQ>
where
  BU: AsMut<[ProcessedJsonRpcResponse<PR>]>
    + cl_aux::Extend<ProcessedJsonRpcResponse<PR>, Error = cl_aux::Error, Output = ()>,
  REQ: Debug
    + PartialOrd
    + Request<
      CP,
      RPD,
      ProcessedResponse = ProcessedJsonRpcResponse<PR>,
      RawResponse = JsonRpcResponse<RR>,
    >,
  RR: Debug + for<'de> Deserialize<'de>,
{
  #[inline]
  fn manage_responses(&self, buffer: &mut BU, bytes: &[u8]) -> crate::Result<()> {
    push_responses_from_reqs_slice(buffer, bytes, self)
  }
}
impl<CP, R, RPD> RequestParams<CP, RPD> for Vec<R>
where
  R: Request<CP, RPD>,
{
  #[inline]
  fn modify_all_params(cp: &mut CP, rpd: RPD) -> crate::Result<()> {
    R::modify_all_params(cp, rpd)
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
fn push_responses_from_reqs_cb<BU, CP, F, ITEM, PR, REQ, RPD, RR>(
  buffer: &mut BU,
  bytes: &[u8],
  mut cb: F,
) -> crate::Result<()>
where
  BU: cl_aux::Extend<ProcessedJsonRpcResponse<PR>, Error = cl_aux::Error, Output = ()>,
  F: FnMut(Id) -> crate::Result<ITEM>,
  ITEM: Borrow<REQ>,
  REQ: Debug
    + Request<
      CP,
      RPD,
      ProcessedResponse = ProcessedJsonRpcResponse<PR>,
      RawResponse = JsonRpcResponse<RR>,
    >,
  RR: Debug + for<'de> Deserialize<'de>,
{
  let mut de = serde_json::Deserializer::from_slice(bytes);
  de.deserialize_seq(SeqVisitor::new(|res: JsonRpcResponse<RR>| {
    let req = cb(res.id)?;
    manage_json_rpc_response(req.borrow(), &res)?;
    buffer.extend([REQ::process_response(res)?])?;
    crate::Result::Ok(())
  }))?;
  Ok(())
}

#[inline]
fn push_responses_from_reqs_slice<BU, CP, PR, REQ, RPD, RR>(
  buffer: &mut BU,
  bytes: &[u8],
  slice: &[REQ],
) -> crate::Result<()>
where
  BU: AsMut<[ProcessedJsonRpcResponse<PR>]>
    + cl_aux::Extend<ProcessedJsonRpcResponse<PR>, Error = cl_aux::Error, Output = ()>,
  REQ: Debug
    + PartialOrd
    + Request<
      CP,
      RPD,
      ProcessedResponse = ProcessedJsonRpcResponse<PR>,
      RawResponse = JsonRpcResponse<RR>,
    >,
  RR: Debug + for<'de> Deserialize<'de>,
{
  if slice.is_empty() {
    return Ok(());
  }
  check_sorted(slice.iter())?;
  let mut de = serde_json::Deserializer::from_slice(bytes);
  let mut idx = 0;
  let mut responses_are_not_sorted = false;
  de.deserialize_seq(SeqVisitor::new(|res: JsonRpcResponse<RR>| {
    let (req, req_idx) = search_slice(idx, res.id, slice)?;
    if idx != req_idx {
      responses_are_not_sorted = true;
    }
    manage_json_rpc_response(req.borrow(), &res)?;
    buffer.extend([REQ::process_response(res)?])?;
    idx = idx.wrapping_add(1);
    crate::Result::Ok(())
  }))?;
  if responses_are_not_sorted {
    buffer.as_mut().sort_unstable();
  }
  Ok(())
}

// First try indexing and then falls back to binary search
fn search_slice<CP, REQ, RPD>(idx: usize, res_id: Id, reqs: &[REQ]) -> crate::Result<(&REQ, usize)>
where
  REQ: Request<CP, RPD>,
{
  let opt = reqs.get(idx).and_then(|req| (req.id() == res_id).then(|| req));
  if let Some(elem) = opt {
    Ok((elem, idx))
  } else {
    reqs
      .binary_search_by(|req| req.id().cmp(&res_id))
      .ok()
      .and_then(|local_idx| Some((reqs.get(local_idx)?, local_idx)))
      .ok_or(crate::Error::JsonRpcResponseIsNotPresentInAnySentRequest(res_id))
  }
}
