use crate::{
  protocol::{JsonRpcResponse, ProcessedJsonRpcResponse},
  types::Id,
  utils::{manage_json_rpc_response, SeqVisitor},
  Request,
};
use alloc::{collections::BTreeSet, vec::Vec};
use arrayvec::ArrayVec;
use cl_aux::SizeHint;
use core::{borrow::Borrow, fmt::Debug};
use serde::{Deserialize, Deserializer};

/// For protocols that allow and process multiple requests in a single call. For example, JSON-RPC.
pub trait Requests<BU>: SizeHint {
  /// Everything but tuples return `()`
  type Output;

  /// Process bytes received from a counterpart and generally store them in the passed `buffer`.
  fn manage_responses(&self, buffer: &mut BU, bytes: &[u8]) -> crate::Result<Self::Output>;
}

impl<BU, PR, REQ, RR> Requests<BU> for [REQ]
where
  BU: AsMut<[ProcessedJsonRpcResponse<PR>]>
    + cl_aux::Extend<ProcessedJsonRpcResponse<PR>, Error = cl_aux::Error, Output = ()>,
  REQ: Debug
    + PartialOrd
    + Request<ProcessedResponse = ProcessedJsonRpcResponse<PR>, RawResponse = JsonRpcResponse<RR>>,
  RR: Debug + for<'de> Deserialize<'de>,
{
  type Output = ();

  #[inline]
  fn manage_responses(&self, buffer: &mut BU, bytes: &[u8]) -> crate::Result<Self::Output> {
    push_responses_from_reqs_slice(buffer, bytes, self)
  }
}

impl<BU, PR, REQ, RR, const N: usize> Requests<BU> for [REQ; N]
where
  BU: AsMut<[ProcessedJsonRpcResponse<PR>]>
    + cl_aux::Extend<ProcessedJsonRpcResponse<PR>, Error = cl_aux::Error, Output = ()>,
  REQ: Debug
    + PartialOrd
    + Request<ProcessedResponse = ProcessedJsonRpcResponse<PR>, RawResponse = JsonRpcResponse<RR>>,
  RR: Debug + for<'de> Deserialize<'de>,
{
  type Output = ();

  #[inline]
  fn manage_responses(&self, buffer: &mut BU, bytes: &[u8]) -> crate::Result<Self::Output> {
    push_responses_from_reqs_slice(buffer, bytes, &self[..])
  }
}

impl<BU, PR, REQ, RR, const N: usize> Requests<BU> for ArrayVec<REQ, N>
where
  BU: AsMut<[ProcessedJsonRpcResponse<PR>]>
    + cl_aux::Extend<ProcessedJsonRpcResponse<PR>, Error = cl_aux::Error, Output = ()>,
  REQ: Debug
    + PartialOrd
    + Request<ProcessedResponse = ProcessedJsonRpcResponse<PR>, RawResponse = JsonRpcResponse<RR>>,
  RR: Debug + for<'de> Deserialize<'de>,
{
  type Output = ();

  #[inline]
  fn manage_responses(&self, buffer: &mut BU, bytes: &[u8]) -> crate::Result<Self::Output> {
    push_responses_from_reqs_slice(buffer, bytes, self)
  }
}

impl<BU, PR, REQ, RR> Requests<BU> for Vec<REQ>
where
  BU: AsMut<[ProcessedJsonRpcResponse<PR>]>
    + cl_aux::Extend<ProcessedJsonRpcResponse<PR>, Error = cl_aux::Error, Output = ()>,
  REQ: Debug
    + PartialOrd
    + Request<ProcessedResponse = ProcessedJsonRpcResponse<PR>, RawResponse = JsonRpcResponse<RR>>,
  RR: Debug + for<'de> Deserialize<'de>,
{
  type Output = ();

  #[inline]
  fn manage_responses(&self, buffer: &mut BU, bytes: &[u8]) -> crate::Result<Self::Output> {
    push_responses_from_reqs_slice(buffer, bytes, self)
  }
}

impl<BU, PR, REQ, RR> Requests<BU> for BTreeSet<REQ>
where
  BU: cl_aux::Extend<ProcessedJsonRpcResponse<PR>, Error = cl_aux::Error, Output = ()>,
  REQ: Borrow<usize>
    + Debug
    + Request<ProcessedResponse = ProcessedJsonRpcResponse<PR>, RawResponse = JsonRpcResponse<RR>>
    + Ord,
  RR: Debug + for<'de> Deserialize<'de>,
{
  type Output = ();

  #[inline]
  fn manage_responses(&self, buffer: &mut BU, bytes: &[u8]) -> crate::Result<Self::Output> {
    if self.is_empty() {
      return Ok(());
    }
    push_responses_from_reqs_cb::<_, _, _, REQ, _>(buffer, bytes, |id| {
      self.get(&id).ok_or(crate::Error::JsonRpcResponseIsNotPresentInAnySentRequest(id))
    })
  }
}

#[cfg(feature = "std")]
impl<BU, PR, REQ, RR> Requests<BU> for std::collections::HashSet<REQ>
where
  BU: AsMut<[ProcessedJsonRpcResponse<PR>]>
    + cl_aux::Extend<ProcessedJsonRpcResponse<PR>, Error = cl_aux::Error, Output = ()>,
  REQ: Borrow<usize>
    + Debug
    + Eq
    + core::hash::Hash
    + Request<ProcessedResponse = ProcessedJsonRpcResponse<PR>, RawResponse = JsonRpcResponse<RR>>,
  RR: Debug + for<'de> Deserialize<'de>,
{
  type Output = ();

  #[inline]
  fn manage_responses(&self, buffer: &mut BU, bytes: &[u8]) -> crate::Result<Self::Output> {
    if self.is_empty() {
      return Ok(());
    }
    push_responses_from_reqs_cb::<_, _, _, REQ, _>(buffer, bytes, |id| {
      self.get(&id).ok_or(crate::Error::JsonRpcResponseIsNotPresentInAnySentRequest(id))
    })
  }
}

impl<BU, T> Requests<BU> for &'_ T
where
  T: Requests<BU>,
{
  type Output = T::Output;

  #[inline]
  fn manage_responses(&self, buffer: &mut BU, bytes: &[u8]) -> crate::Result<Self::Output> {
    T::manage_responses(self, buffer, bytes)
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
fn push_responses_from_reqs_cb<BU, PR, ITEM, REQ, RR>(
  buffer: &mut BU,
  bytes: &[u8],
  mut cb: impl FnMut(Id) -> crate::Result<ITEM>,
) -> crate::Result<()>
where
  BU: cl_aux::Extend<ProcessedJsonRpcResponse<PR>, Error = cl_aux::Error, Output = ()>,
  ITEM: Borrow<REQ>,
  REQ: Debug
    + Request<ProcessedResponse = ProcessedJsonRpcResponse<PR>, RawResponse = JsonRpcResponse<RR>>,
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
fn push_responses_from_reqs_slice<BU, PR, REQ, RR>(
  buffer: &mut BU,
  bytes: &[u8],
  slice: &[REQ],
) -> crate::Result<()>
where
  BU: AsMut<[ProcessedJsonRpcResponse<PR>]>
    + cl_aux::Extend<ProcessedJsonRpcResponse<PR>, Error = cl_aux::Error, Output = ()>,
  REQ: Debug
    + PartialOrd
    + Request<ProcessedResponse = ProcessedJsonRpcResponse<PR>, RawResponse = JsonRpcResponse<RR>>,
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
fn search_slice<REQ>(idx: usize, res_id: Id, reqs: &[REQ]) -> crate::Result<(&REQ, usize)>
where
  REQ: Request,
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
