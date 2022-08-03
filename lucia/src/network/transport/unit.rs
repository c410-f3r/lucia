use crate::{
  dnsn::Serialize,
  network::Transport,
  req_res::{RequestManager, RequestParamsModifier},
};
use alloc::{boxed::Box, vec::Vec};

/// Does absolutely nothing. Good for demonstration purposes.
///
/// ```rust,no_run
/// # async fn fun() -> lucia::Result<()> {
/// use lucia::{
///   misc::{CommonParams, Pair},
///   network::Transport,
///   req_res::RequestManager
/// };
/// let (mut rm, mut trans) = Pair::new(
///   RequestManager::new((), CommonParams::default(), ()), ()
/// ).into_parts();
/// let req = ();
/// let _res = trans.send_retrieve_and_decode_one(&mut rm, &req, ()).await?;
/// # Ok(()) }
/// ```
#[async_trait::async_trait]
impl<A, CP, DRSR> Transport<A, CP, DRSR> for ()
where
  A: Send,
  CP: Send,
  DRSR: Send,
{
  type ResponseParams = ();

  #[inline]
  async fn send<REQ, REQP>(
    &mut self,
    _: &mut RequestManager<A, CP, DRSR>,
    _: &REQ,
    _: REQP,
  ) -> Result<Self::ResponseParams, REQ::Error>
  where
    REQ: RequestParamsModifier<CP, REQP> + Send + Serialize<DRSR> + Sync,
    REQP: Send,
  {
    Ok(())
  }

  #[inline]
  async fn send_and_retrieve<REQ, REQP>(
    &mut self,
    _: &mut RequestManager<A, CP, DRSR>,
    _: &REQ,
    _: REQP,
  ) -> Result<(Self::ResponseParams, Vec<u8>), REQ::Error>
  where
    REQ: RequestParamsModifier<CP, REQP> + Send + Serialize<DRSR> + Sync,
    REQP: Send,
  {
    Ok(((), Vec::new()))
  }
}
