use crate::{network::Transport, RequestManager, RequestParams};
use alloc::{boxed::Box, vec::Vec};
use core::fmt::Debug;
use serde::Serialize;

/// Does absolutely nothing. Good for demonstration purposes.
///
/// ```rust,no_run
/// # async fn fun() -> lucia::Result<()> {
/// use lucia::{
///   network::Transport,
///   Pair,
/// };
/// let (mut rm, mut trans) = Pair::<(), _, _>::new((), ()).into_parts();
/// let req = ();
/// let _res = trans.send_retrieve_and_decode_one(&mut rm, &req, ()).await?;
/// Ok(())
/// # }
/// ```
#[async_trait::async_trait]
impl<A, CP> Transport<A, CP> for ()
where
  A: Send,
  CP: Send,
{
  #[inline]
  async fn send<R, RPD>(&mut self, _: &mut RequestManager<A, CP>, _: R, _: RPD) -> crate::Result<()>
  where
    R: Debug + RequestParams<CP, RPD> + Send + Serialize + Sync,
    RPD: Send,
  {
    Ok(())
  }

  #[inline]
  async fn send_and_retrieve<R, RPD>(
    &mut self,
    _: &mut RequestManager<A, CP>,
    _: R,
    _: RPD,
  ) -> crate::Result<Vec<u8>>
  where
    R: Debug + RequestParams<CP, RPD> + Send + Serialize + Sync,
    RPD: Send,
  {
    Ok(Vec::new())
  }
}
