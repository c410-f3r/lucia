use crate::{
  misc::log_req,
  network::{transport::Transport, TransportGroup},
  package::{Package, PackagesAux},
};
use alloc::boxed::Box;

/// Does absolutely nothing. Good for demonstration purposes.
///
/// ```rust,no_run
/// # async fn fun() -> lucia::Result<()> {
/// use lucia::{network::transport::Transport, package::PackagesAux};
/// let _ =
///   ().send_retrieve_and_decode_contained(&mut (), &mut PackagesAux::from_minimum((), (), ()))
///     .await?;
/// # Ok(()) }
/// ```
#[async_trait::async_trait]
impl<DRSR> Transport<DRSR> for ()
where
  DRSR: Send + Sync,
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
    log_req(pkg, pkgs_aux, self);
    Ok(())
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
    log_req(pkg, pkgs_aux, self);
    Ok(0)
  }
}
