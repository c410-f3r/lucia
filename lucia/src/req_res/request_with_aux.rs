use crate::{
  data_formats::JsonRpcRequest,
  misc::FromErrorTy,
  req_res::{RequestParamsModifier, RequestResponse},
  Id,
};
use core::{
  borrow::Borrow,
  cmp::{Ord, Ordering},
  hash::{Hash, Hasher},
  marker::PhantomData,
};

/// Used to store any type of auxiliary data along side a request. Auxiliary is not
/// serialized nor deserialized.
///
/// # Types
///
/// * `AUX`: **AUX**iliary
/// * `CP`: **C**ommon **P**arameters
/// * `REQ`: **REQ**equest
/// * `REQP`: **REQ**uest **P**arameters
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Debug)]
pub struct RequestWithAux<AUX, CP, REQ, REQP, RESP> {
  /// Auxiliary
  #[cfg_attr(feature = "serde", serde(skip))]
  pub aux: AUX,
  /// Request
  pub req: REQ,
  phantom: PhantomData<(CP, REQP, RESP)>,
}

impl<AUX, CP, REQ, REQP, RESP> RequestWithAux<AUX, CP, REQ, REQP, RESP> {
  /// Constructor shortcut
  #[inline]
  pub fn new(aux: AUX, req: REQ) -> Self {
    Self { aux, req, phantom: PhantomData }
  }
}

impl<AUX, CP, REQ, REQP, RESP> FromErrorTy for RequestWithAux<AUX, CP, REQ, REQP, RESP>
where
  REQ: FromErrorTy,
{
  type Error = REQ::Error;
}

impl<AUX, CP, REQ, REQP, RESP> RequestParamsModifier<CP, REQP>
  for RequestWithAux<AUX, CP, REQ, REQP, RESP>
where
  REQ: RequestParamsModifier<CP, REQP> + RequestResponse<RESP>,
{
  #[inline]
  fn modify_all_params(cp: &mut CP, reqp: REQP) -> Result<(), Self::Error> {
    REQ::modify_all_params(cp, reqp)
  }
}

impl<AUX, CP, REQ, REQP, RESP> RequestResponse<RESP> for RequestWithAux<AUX, CP, REQ, REQP, RESP>
where
  REQ: RequestParamsModifier<CP, REQP> + RequestResponse<RESP>,
{
  type ProcessedResponse = REQ::ProcessedResponse;
  type RawResponse = REQ::RawResponse;

  #[inline]
  fn process(raw: Self::RawResponse, resp: &RESP) -> Result<Self::ProcessedResponse, Self::Error> {
    REQ::process(raw, resp)
  }
}

impl<AUX, CP, RP, REQP, RESP> Borrow<Id>
  for RequestWithAux<AUX, CP, JsonRpcRequest<RP>, REQP, RESP>
{
  #[inline]
  fn borrow(&self) -> &Id {
    &self.req.id
  }
}

impl<AUX, CP, REQ, REQP, RESP> Eq for RequestWithAux<AUX, CP, REQ, REQP, RESP> where REQ: Eq {}

impl<AUX, CP, REQ, REQP, RESP> Hash for RequestWithAux<AUX, CP, REQ, REQP, RESP>
where
  REQ: Hash,
{
  #[inline]
  fn hash<H>(&self, state: &mut H)
  where
    H: Hasher,
  {
    self.req.hash(state);
  }
}

impl<AUX, CP, REQ, REQP, RESP> Ord for RequestWithAux<AUX, CP, REQ, REQP, RESP>
where
  REQ: Ord,
{
  #[inline]
  fn cmp(&self, other: &Self) -> Ordering {
    self.req.cmp(&other.req)
  }
}

impl<AUX, CP, REQ, REQP, RESP> PartialEq for RequestWithAux<AUX, CP, REQ, REQP, RESP>
where
  REQ: PartialEq,
{
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.req == other.req
  }
}

impl<AUX, CP, REQ, REQP, RESP> PartialOrd for RequestWithAux<AUX, CP, REQ, REQP, RESP>
where
  REQ: PartialOrd,
{
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.req.partial_cmp(&other.req)
  }
}
