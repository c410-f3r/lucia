use core::ops::{Deref, DerefMut};
use lucia::req_res::RequestManager;

/// Just a wrapper that implements [Deref] and [DerefMut] to easily call method from
/// [RequestManager].
#[derive(Debug)]
pub struct RequestManagerWrapper<A, CP, DRSR>(RequestManager<A, CP, DRSR>);

impl<A, CP, DRSR> RequestManagerWrapper<A, CP, DRSR> {
  /// Proxy of [RequestManager].
  #[inline]
  pub const fn new(api: A, cp: CP, drsr: DRSR) -> Self {
    Self(RequestManager::new(api, cp, drsr))
  }
}

impl<A, CP, DRSR> Deref for RequestManagerWrapper<A, CP, DRSR> {
  type Target = RequestManager<A, CP, DRSR>;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<A, CP, DRSR> DerefMut for RequestManagerWrapper<A, CP, DRSR> {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
