use crate::{
  misc::manage_before_sending_related,
  network::{transport::Transport, TransportGroup},
  package::{Package, PackagesAux},
};
#[cfg(not(feature = "async-fn-in-trait"))]
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
#[cfg_attr(not(feature = "async-fn-in-trait"), async_trait::async_trait)]
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
    manage_before_sending_related(pkg, pkgs_aux, self).await?;
    pkg.after_sending(&mut pkgs_aux.api, &mut pkgs_aux.ext_res_params).await?;
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
    self.send(pkg, pkgs_aux).await?;
    Ok(0)
  }
}
