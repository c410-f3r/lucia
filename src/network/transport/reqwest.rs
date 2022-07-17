use crate::{
  network::{HttpMethod, HttpParamsMut, Transport},
  RequestManager, RequestParams,
};
use alloc::vec::Vec;
use core::fmt::Debug;
use reqwest::{Client, Response};
use serde::Serialize;

/// Handy constructor to avoid having to explicitly import `reqwest`.
#[inline]
pub fn reqwest() -> Client {
  <_>::default()
}

/// ```rust,no_run
/// # async fn fun() -> lucia::Result<()> {
/// use lucia::{
///   network::{reqwest, HttpParams, Transport},
///   Pair,
/// };
/// let (mut rm, mut trans) = Pair::<(), _, _>::new(
///   reqwest(),
///   HttpParams::from_origin("ORIGIN")?
/// ).into_parts();
/// let req = ();
/// let _res = trans.send_retrieve_and_decode_one(&mut rm, &req, ()).await?;
/// Ok(())
/// # }
/// ```
#[async_trait::async_trait]
impl<A, CP> Transport<A, CP> for Client
where
  A: Send,
  CP: Send,
  for<'rm> &'rm mut CP: Into<HttpParamsMut<'rm>>,
{
  #[inline]
  async fn send<R, RPD>(
    &mut self,
    rm: &mut RequestManager<A, CP>,
    req: R,
    rpd: RPD,
  ) -> crate::Result<()>
  where
    R: Debug + RequestParams<CP, RPD> + Send + Serialize + Sync,
    RPD: Send,
  {
    R::modify_all_params(&mut rm._cp, rpd)?;
    let _ = reqwest_send(self, req, (&mut rm._cp).into()).await;
    Ok(())
  }

  #[inline]
  async fn send_and_retrieve<R, RPD>(
    &mut self,
    rm: &mut RequestManager<A, CP>,
    req: R,
    rpd: RPD,
  ) -> crate::Result<Vec<u8>>
  where
    R: Debug + RequestParams<CP, RPD> + Send + Serialize + Sync,
    RPD: Send,
  {
    R::modify_all_params(&mut rm._cp, rpd)?;
    Ok(reqwest_send(self, req, (&mut rm._cp).into()).await?.bytes().await?.to_vec())
  }
}

#[inline]
async fn reqwest_send<T>(
  client: &Client,
  req: T,
  mut params: HttpParamsMut<'_>,
) -> crate::Result<Response>
where
  T: Debug + Send + Serialize + Sync,
{
  if let Some(ref mut rt) = params._rt {
    rt.rc.update_params(&rt.rl).await?;
  }
  let mut req = match params._method {
    HttpMethod::_Get => client.get(params._url_parts.url()),
    HttpMethod::_Post => client.post(params._url_parts.url()).json(&req),
  };
  for header in params._headers.iter() {
    req = req.header(header._key.as_str(), header._value.as_str());
  }
  params._reset();
  Ok(req.send().await?)
}
