#![allow(
  // Intended for testing environments
  clippy::indexing_slicing
)]

use crate::{
  misc::{manage_before_sending_related, FromBytes},
  network::{transport::Transport, TransportGroup},
  pkg::{Package, PkgsAux},
};
use alloc::{
  borrow::{Cow, ToOwned},
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
///   pkg::PkgsAux,
/// };
/// let _ = MockStr::default()
///   .send_retrieve_and_decode_contained(&mut (), &mut PkgsAux::from_minimum((), (), ()))
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
  ///
  /// # Panics
  ///
  /// If the number of asserted requests differs from the number of stored requests.
  #[inline]
  pub fn assert_does_not_have_non_asserted_requests(&self) {
    assert_eq!(self.asserted, self.requests.len());
  }

  /// Verifies if `req` is present in the inner request storage.
  ///
  /// # Panics
  ///
  /// If the stored request differs from the passed request.
  #[inline]
  #[track_caller]
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

  fn pop_response(&mut self) -> crate::Result<Cow<'static, T>> {
    self.responses.pop_front().ok_or(crate::Error::TestTransportNoResponse)
  }
}

impl<DRSR, T> Transport<DRSR> for Mock<T>
where
  T: AsRef<[u8]> + Debug + PartialEq + ToOwned + 'static + ?Sized,
  <T as ToOwned>::Owned: Debug + FromBytes,
{
  const GROUP: TransportGroup = TransportGroup::Stub;
  type Params = ();

  #[inline]
  async fn send<P>(
    &mut self,
    pkg: &mut P,
    pkgs_aux: &mut PkgsAux<P::Api, DRSR, Self::Params>,
  ) -> Result<(), P::Error>
  where
    P: Package<DRSR, ()>,
  {
    manage_before_sending_related(pkg, pkgs_aux, self).await?;
    self.requests.push(Cow::Owned(FromBytes::from_bytes(&pkgs_aux.byte_buffer)?));
    pkgs_aux.byte_buffer.clear();
    pkg.after_sending(&mut pkgs_aux.api, &mut ()).await?;
    Ok(())
  }

  #[inline]
  async fn send_and_retrieve<P>(
    &mut self,
    pkg: &mut P,
    pkgs_aux: &mut PkgsAux<P::Api, DRSR, Self::Params>,
  ) -> Result<usize, P::Error>
  where
    P: Package<DRSR, ()>,
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
