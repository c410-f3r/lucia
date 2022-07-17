#![allow(
  // Intended for testing environments
  clippy::indexing_slicing
)]

use crate::{network::Transport, RequestManager, RequestParams};
use alloc::{borrow::Cow, boxed::Box, collections::VecDeque, vec::Vec};
use core::fmt::Debug;
use serde::Serialize;

/// Used to assert issued requests as well as returned responses in a local environment. Intended
/// mostly for testing.
///
/// Almost all methods panic at runtime.
///
/// ```rust,no_run
/// # async fn fun() -> lucia::Result<()> {
/// use lucia::{
///   network::{Test, Transport},
///   Pair,
/// };
/// let (mut rm, mut trans) = Pair::<(), _, _>::new(Test::default(), ()).into_parts();
/// let req = ();
/// let _res = trans.send_retrieve_and_decode_one(&mut rm, &req, ()).await?;
/// Ok(())
/// # }
/// ```
#[derive(Debug, Default)]
pub struct Test {
  asserted: usize,
  requests: Vec<Cow<'static, str>>,
  responses: VecDeque<Cow<'static, str>>,
}

impl Test {
  /// Ensures that no included request hasn't processed.
  #[inline]
  pub fn assert_does_not_have_non_asserted_requests(&self) {
    assert_eq!(self.asserted, self.requests.len())
  }

  /// Verifies if `req` is present in the inner request storage.
  #[inline]
  pub fn assert_request(&mut self, req: &str) {
    let stored = &self.requests[self.asserted];
    self.asserted = self.asserted.wrapping_add(1);
    assert_eq!(req, stored);
  }

  /// Stores `res` into the inner response storage
  #[inline]
  pub fn push_response(&mut self, res: Cow<'static, str>) {
    self.responses.push_back(res);
  }

  #[inline]
  fn pop_response(&mut self) -> crate::Result<Cow<'static, str>> {
    self.responses.pop_front().ok_or(crate::Error::TestTransportNoResponse)
  }

  #[inline]
  fn store_req<T>(&mut self, req: T) -> crate::Result<()>
  where
    T: Debug + Send + Serialize,
  {
    let serialized = serde_json::to_string(&req)?;
    self.requests.push(serialized.into());
    Ok(())
  }
}

#[async_trait::async_trait]
impl<A, CP> Transport<A, CP> for Test
where
  A: Send,
  CP: Send,
{
  #[inline]
  async fn send<R, RPD>(
    &mut self,
    _: &mut RequestManager<A, CP>,
    req: R,
    _: RPD,
  ) -> crate::Result<()>
  where
    R: Debug + RequestParams<CP, RPD> + Send + Serialize + Sync,
    RPD: Send,
  {
    self.store_req(req)
  }

  #[inline]
  async fn send_and_retrieve<R, RPD>(
    &mut self,
    _: &mut RequestManager<A, CP>,
    req: R,
    _: RPD,
  ) -> crate::Result<Vec<u8>>
  where
    R: Debug + RequestParams<CP, RPD> + Send + Serialize + Sync,
    RPD: Send,
  {
    self.store_req(req)?;
    let response = self.pop_response()?;
    Ok(response.as_bytes().to_vec())
  }
}
