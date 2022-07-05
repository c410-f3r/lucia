use crate::{types::Id, Request};
use core::{
  cmp::{Ord, Ordering},
  hash::{Hash, Hasher},
};

/// Used to store any type of auxiliary data along side a request. Auxiliary is not
/// serialized nor deserialized.
#[derive(Debug, serde::Serialize)]
#[serde(transparent)]
pub struct RequestWithAux<A, R> {
  /// Auxiliary
  #[serde(skip)]
  pub aux: A,
  /// Request
  pub req: R,
}

impl<A, R> Request for RequestWithAux<A, R>
where
  R: Request,
{
  type ProcessedResponse = R::ProcessedResponse;
  type RawResponse = R::RawResponse;

  #[inline]
  fn id(&self) -> Id {
    self.req.id()
  }

  #[inline]
  fn process_response(raw: Self::RawResponse) -> crate::Result<Self::ProcessedResponse> {
    R::process_response(raw)
  }
}

impl<A, R> Eq for RequestWithAux<A, R> where R: Request {}

impl<A, R> Hash for RequestWithAux<A, R>
where
  R: Request,
{
  #[inline]
  fn hash<H>(&self, state: &mut H)
  where
    H: Hasher,
  {
    self.id().hash(state);
  }
}

impl<A, R> Ord for RequestWithAux<A, R>
where
  R: Request,
{
  #[inline]
  fn cmp(&self, other: &Self) -> Ordering {
    self.id().cmp(&other.id())
  }
}

impl<A, R> PartialEq for RequestWithAux<A, R>
where
  R: Request,
{
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.id() == other.id()
  }
}

impl<A, R> PartialOrd for RequestWithAux<A, R>
where
  R: Request,
{
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}
