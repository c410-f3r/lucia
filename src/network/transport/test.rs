#![allow(
  // Intended for testing environments
  clippy::indexing_slicing
)]

use crate::{
  dnsn::Serialize, network::Transport, utils::FromBytes, RequestManager, RequestParamsModifier,
};
use alloc::{
  borrow::{Cow, ToOwned},
  boxed::Box,
  collections::VecDeque,
  vec::Vec,
};
use core::fmt::Debug;

/// Used to assert issued requests as well as returned responses in a local environment. Intended
/// mostly for testing.
///
/// Almost all methods panic at runtime.
///
/// ```rust,no_run
/// # async fn fun() -> lucia::Result<()> {
/// use lucia::{
///   network::{Test, Transport},
///   CommonParams, Pair, RequestManager
/// };
/// let (mut rm, mut trans) = Pair::new(
///   RequestManager::new(
///     (),
///     CommonParams::default(),
///     (),
///   ),
///   Test::<str>::default()
/// ).into_parts();
/// let req = ();
/// let _res = trans.send_retrieve_and_decode_one(&mut rm, &req, ()).await?;
/// # Ok(()) }
/// ```
#[derive(Debug)]
pub struct Test<B>
where
  B: ToOwned + 'static + ?Sized,
  <B as ToOwned>::Owned: Debug + FromBytes,
{
  asserted: usize,
  requests: Vec<Cow<'static, B>>,
  responses: VecDeque<Cow<'static, B>>,
}

impl<B> Test<B>
where
  B: AsRef<[u8]> + Debug + PartialEq + ToOwned + 'static + ?Sized,
  <B as ToOwned>::Owned: Debug + FromBytes,
{
  /// Ensures that no included request hasn't processed.
  #[inline]
  pub fn assert_does_not_have_non_asserted_requests(&self) {
    assert_eq!(self.asserted, self.requests.len())
  }

  /// Verifies if `req` is present in the inner request storage.
  #[inline]
  pub fn assert_request(&mut self, req: &B) {
    let stored = &self.requests[self.asserted];
    self.asserted = self.asserted.wrapping_add(1);
    assert_eq!(req, stored.as_ref());
  }

  /// Stores `res` into the inner response storage
  #[inline]
  pub fn push_response(&mut self, res: Cow<'static, B>) {
    self.responses.push_back(res);
  }

  #[inline]
  fn pop_response(&mut self) -> crate::Result<Cow<'static, B>> {
    self.responses.pop_front().ok_or(crate::Error::TestTransportNoResponse)
  }

  #[inline]
  fn store_req<DRSR, T>(&mut self, drsr: &mut DRSR, req: &T) -> crate::Result<()>
  where
    T: Send + Serialize<DRSR>,
  {
    let mut vec = Vec::new();
    req.to_bytes(&mut vec, drsr)?;
    self.requests.push(Cow::Owned(FromBytes::from_bytes(&vec)?));
    Ok(())
  }
}

#[async_trait::async_trait]
impl<A, B, CP, DRSR> Transport<A, CP, DRSR> for Test<B>
where
  A: Send,
  B: AsRef<[u8]> + Debug + PartialEq + ToOwned + Send + Sync + 'static + ?Sized,
  CP: Send,
  DRSR: Send,
  <B as ToOwned>::Owned: Debug + FromBytes + Send,
{
  type Metadata = ();

  #[inline]
  async fn send<REQ, REQP>(
    &mut self,
    rm: &mut RequestManager<A, CP, DRSR>,
    req: &REQ,
    _: REQP,
  ) -> crate::Result<()>
  where
    REQ: RequestParamsModifier<CP, REQP> + Send + Serialize<DRSR> + Sync,
    REQP: Send,
  {
    self.store_req(&mut rm.drsr, req)
  }

  #[inline]
  async fn send_and_retrieve<REQ, REQP>(
    &mut self,
    rm: &mut RequestManager<A, CP, DRSR>,
    req: &REQ,
    _: REQP,
  ) -> crate::Result<(Self::Metadata, Vec<u8>)>
  where
    REQ: RequestParamsModifier<CP, REQP> + Send + Serialize<DRSR> + Sync,
    REQP: Send,
  {
    self.store_req(&mut rm.drsr, req)?;
    let response = self.pop_response()?;
    let bytes: &[u8] = response.as_ref().as_ref();
    Ok(((), bytes.into()))
  }
}

impl<B> Default for Test<B>
where
  B: ToOwned + 'static + ?Sized,
  <B as ToOwned>::Owned: Debug + FromBytes,
{
  #[inline]
  fn default() -> Self {
    Self { asserted: 0, requests: Vec::new(), responses: VecDeque::new() }
  }
}
