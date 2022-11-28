#![allow(
  // Intended for testing environments
  clippy::indexing_slicing
)]

use crate::{
  dnsn::Serialize,
  misc::{log_req, FromBytes},
  network::{transport::Transport, TransportGroup},
  package::{Package, PackagesAux},
};
use alloc::{
  borrow::{Cow, ToOwned},
  boxed::Box,
  collections::VecDeque,
  vec::Vec,
};
use core::fmt::Debug;

/// For API's that send and received raw bytes.
pub type MockBytes = Mock<[u8]>;
/// For API's that send and received strings.
pub type MockStr = Mock<str>;

/// Used to assert issued requests as well as returned responses in a local environment.
///
/// Almost all methods panic at runtime.
///
/// ```rust,no_run
/// # async fn fun() -> lucia::Result<()> {
/// use lucia::{
///   network::transport::{MockStr, Transport},
///   package::PackagesAux,
/// };
/// let _ = MockStr::default()
///   .send_retrieve_and_decode_contained(&mut (), &mut PackagesAux::from_minimum((), (), ()))
///   .await?;
/// # Ok(()) }
/// ```
#[derive(Debug)]
pub struct Mock<T>
where
  T: ToOwned + 'static + ?Sized,
  <T as ToOwned>::Owned: Debug + FromBytes,
{
  asserted: usize,
  requests: Vec<Cow<'static, T>>,
  responses: VecDeque<Cow<'static, T>>,
}

impl<T> Mock<T>
where
  T: AsRef<[u8]> + Debug + PartialEq + ToOwned + 'static + ?Sized,
  <T as ToOwned>::Owned: Debug + FromBytes,
{
  /// Ensures that no included request hasn't processed.
  #[inline]
  pub fn assert_does_not_have_non_asserted_requests(&self) {
    assert_eq!(self.asserted, self.requests.len())
  }

  /// Verifies if `req` is present in the inner request storage.
  #[inline]
  pub fn assert_request(&mut self, req: &T) {
    let stored = &self.requests[self.asserted];
    self.asserted = self.asserted.wrapping_add(1);
    assert_eq!(req, stored.as_ref());
  }

  /// Stores `res` into the inner response storage
  #[inline]
  pub fn push_response(&mut self, res: Cow<'static, T>) {
    self.responses.push_back(res);
  }

  #[inline]
  fn pop_response(&mut self) -> crate::Result<Cow<'static, T>> {
    self.responses.pop_front().ok_or(crate::Error::TestTransportNoResponse)
  }
}

#[async_trait::async_trait]
impl<DRSR, T> Transport<DRSR> for Mock<T>
where
  DRSR: Send + Sync,
  T: AsRef<[u8]> + Debug + PartialEq + ToOwned + Send + Sync + 'static + ?Sized,
  <T as ToOwned>::Owned: Debug + FromBytes + Send + Sync,
{
  const GROUP: TransportGroup = TransportGroup::Stub;
  type Params = ();

  #[inline]
  async fn send<P>(
    &mut self,
    pkg: &mut P,
    pkgs_aux: &mut PackagesAux<P::Api, DRSR, Self::Params>,
  ) -> Result<(), P::Error>
  where
    P: Package<DRSR, ()> + Send + Sync,
  {
    let rslt = {
      let mut vec = Vec::new();
      pkg.ext_req_ctnt_mut().to_bytes(&mut vec, &mut pkgs_aux.drsr)?;
      self.requests.push(Cow::Owned(FromBytes::from_bytes(&vec)?))
    };
    log_req(pkg, pkgs_aux, self);
    Ok(rslt)
  }

  #[inline]
  async fn send_and_retrieve<P>(
    &mut self,
    pkg: &mut P,
    pkgs_aux: &mut PackagesAux<P::Api, DRSR, Self::Params>,
  ) -> Result<usize, P::Error>
  where
    P: Package<DRSR, ()> + Send + Sync,
  {
    <Self as Transport<DRSR>>::send(self, pkg, pkgs_aux).await?;
    let response = self.pop_response()?;
    pkgs_aux.byte_buffer.clear();
    pkgs_aux.byte_buffer.extend(response.as_ref().as_ref().iter().copied());
    Ok(pkgs_aux.byte_buffer.len())
  }
}

impl<BB> Default for Mock<BB>
where
  BB: ToOwned + 'static + ?Sized,
  <BB as ToOwned>::Owned: Debug + FromBytes,
{
  #[inline]
  fn default() -> Self {
    Self { asserted: 0, requests: Vec::new(), responses: VecDeque::new() }
  }
}
