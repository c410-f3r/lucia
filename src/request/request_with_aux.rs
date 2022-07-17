use crate::{types::Id, Request, RequestData, RequestParams};
use core::{
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
/// * `R`: **R**equest
/// * `RPD`: **R**equest **P**arameters **D**efinitions
#[derive(Debug, serde::Serialize)]
#[serde(transparent)]
pub struct RequestWithAux<AUX, CP, R, RPD> {
  /// Auxiliary
  #[serde(skip)]
  pub aux: AUX,
  /// Request
  pub req: R,
  phantom: PhantomData<(CP, RPD)>,
}

impl<AUX, CP, R, RPD> RequestWithAux<AUX, CP, R, RPD> {
  /// Constructor shortcut
  #[inline]
  pub fn new(aux: AUX, req: R) -> Self {
    Self { aux, req, phantom: PhantomData }
  }
}

impl<AUX, CP, R, RPD> RequestData for RequestWithAux<AUX, CP, R, RPD>
where
  R: RequestData,
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

impl<AUX, CP, R, RPD> RequestParams<CP, RPD> for RequestWithAux<AUX, CP, R, RPD>
where
  R: RequestParams<CP, RPD>,
{
  #[inline]
  fn modify_all_params(cp: &mut CP, rpd: RPD) -> crate::Result<()> {
    R::modify_all_params(cp, rpd)
  }
}

impl<AUX, CP, R, RPD> Eq for RequestWithAux<AUX, CP, R, RPD> where R: Request<CP, RPD> {}

impl<AUX, CP, R, RPD> Hash for RequestWithAux<AUX, CP, R, RPD>
where
  R: Request<CP, RPD>,
{
  #[inline]
  fn hash<H>(&self, state: &mut H)
  where
    H: Hasher,
  {
    self.id().hash(state);
  }
}

impl<AUX, CP, R, RPD> Ord for RequestWithAux<AUX, CP, R, RPD>
where
  R: Request<CP, RPD>,
{
  #[inline]
  fn cmp(&self, other: &Self) -> Ordering {
    self.id().cmp(&other.id())
  }
}

impl<AUX, CP, R, RPD> PartialEq for RequestWithAux<AUX, CP, R, RPD>
where
  R: Request<CP, RPD>,
{
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.id() == other.id()
  }
}

impl<AUX, CP, R, RPD> PartialOrd for RequestWithAux<AUX, CP, R, RPD>
where
  R: Request<CP, RPD>,
{
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}
