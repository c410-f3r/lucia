use crate::{
  data_formats::JsonRpcRequest,
  misc::FromErrorTy,
  req_res::{Request, RequestParamsModifier},
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
/// * `AUX`: `AUX`iliary
/// * `CP`: **C**ommon **P**arameters
/// * `REQ`: `REQ`equest
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Debug)]
pub struct RequestWithAux<AUX, CP, REQ> {
  /// Auxiliary
  #[cfg_attr(feature = "serde", serde(skip))]
  pub aux: AUX,
  /// Request
  pub req: REQ,
  #[cfg_attr(feature = "serde", serde(skip))]
  phantom: PhantomData<CP>,
}

impl<AUX, CP, REQ> RequestWithAux<AUX, CP, REQ> {
  /// Constructor shortcut
  #[inline]
  pub fn new(aux: AUX, req: REQ) -> Self {
    Self { aux, req, phantom: PhantomData }
  }
}

impl<AUX, CP, REQ> FromErrorTy for RequestWithAux<AUX, CP, REQ>
where
  REQ: FromErrorTy,
{
  type Error = REQ::Error;
}

impl<AUX, CP, REQ, REQP> RequestParamsModifier<CP, REQP> for RequestWithAux<AUX, CP, REQ>
where
  REQ: RequestParamsModifier<CP, REQP>,
{
  #[inline]
  fn modify_all_params(cp: &mut CP, reqp: REQP) -> Result<(), Self::Error> {
    REQ::modify_all_params(cp, reqp)
  }
}

impl<AUX, CP, DRSR, REQ, REQP, RESP> Request<CP, DRSR, REQP, RESP> for RequestWithAux<AUX, CP, REQ>
where
  REQ: Request<CP, DRSR, REQP, RESP>,
{
  type Data = REQ::Data;
  type ProcessedResponse = REQ::ProcessedResponse;
  type RawResponse = REQ::RawResponse;

  #[inline]
  fn data(&self) -> &Self::Data {
    self.req.data()
  }

  #[inline]
  fn process(raw: Self::RawResponse, resp: &RESP) -> Result<Self::ProcessedResponse, Self::Error> {
    REQ::process(raw, resp)
  }
}

impl<AUX, CP, RP> Borrow<Id> for RequestWithAux<AUX, CP, JsonRpcRequest<RP>> {
  #[inline]
  fn borrow(&self) -> &Id {
    &self.req.id
  }
}

impl<AUX, CP, REQ> Eq for RequestWithAux<AUX, CP, REQ> where REQ: Eq {}

impl<AUX, CP, REQ> Hash for RequestWithAux<AUX, CP, REQ>
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

impl<AUX, CP, REQ> Ord for RequestWithAux<AUX, CP, REQ>
where
  REQ: Ord,
{
  #[inline]
  fn cmp(&self, other: &Self) -> Ordering {
    self.req.cmp(&other.req)
  }
}

impl<AUX, CP, REQ> PartialEq for RequestWithAux<AUX, CP, REQ>
where
  REQ: PartialEq,
{
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.req == other.req
  }
}

impl<AUX, CP, REQ> PartialOrd for RequestWithAux<AUX, CP, REQ>
where
  REQ: PartialOrd,
{
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.req.partial_cmp(&other.req)
  }
}
