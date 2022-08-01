use crate::{
  dnsn::Serialize,
  network::{
    http::{AllReqParamsMut, Method},
    Transport,
  },
  RequestManager, RequestParamsModifier,
};
use alloc::vec::Vec;
use reqwest::{
  header::{HeaderValue, CONTENT_TYPE, USER_AGENT},
  Client, RequestBuilder,
};

/// Handy constructor to avoid having to explicitly import `reqwest`.
#[inline]
pub fn reqwest() -> Client {
  <_>::default()
}

/// ```rust,no_run
/// # async fn fun() -> lucia::Result<()> {
/// use lucia::{
///   network::{http::ReqParams, reqwest, Transport},
///   CommonParams, Pair, RequestManager
/// };
/// let (mut rm, mut trans) = Pair::new(
///   RequestManager::new((), CommonParams::new(ReqParams::from_origin("ORIGIN")?, ()), ()),
///   (),
/// ).into_parts();
/// let req = ();
/// let _res = trans.send_retrieve_and_decode_one(&mut rm, &req, ()).await?;
/// # Ok(()) }
/// ```
#[async_trait::async_trait]
impl<A, CP, DRSR> Transport<A, CP, DRSR> for Client
where
  A: Send,
  CP: Send,
  DRSR: Send,
  for<'any> &'any mut CP: Into<AllReqParamsMut<'any>>,
{
  type Metadata = crate::network::http::Response;

  #[inline]
  async fn send<REQ, REQP>(
    &mut self,
    rm: &mut RequestManager<A, CP, DRSR>,
    req: &REQ,
    reqp: REQP,
  ) -> crate::Result<Self::Metadata>
  where
    REQ: RequestParamsModifier<CP, REQP> + Send + Serialize<DRSR> + Sync,
    REQP: Send,
  {
    REQ::modify_all_params(&mut rm.cp, reqp)?;
    let res = reqwest_response((&mut rm.cp).into(), self, &mut rm.drsr, req).await?;
    Ok((&res).try_into()?)
  }

  #[inline]
  async fn send_and_retrieve<REQ, REQP>(
    &mut self,
    rm: &mut RequestManager<A, CP, DRSR>,
    req: &REQ,
    reqp: REQP,
  ) -> crate::Result<(Self::Metadata, Vec<u8>)>
  where
    REQ: RequestParamsModifier<CP, REQP> + Send + Serialize<DRSR> + Sync,
    REQP: Send,
  {
    REQ::modify_all_params(&mut rm.cp, reqp)?;
    let res = reqwest_response((&mut rm.cp).into(), self, &mut rm.drsr, req).await?;
    Ok(((&res).try_into()?, res.bytes().await?.into()))
  }
}

#[inline]
async fn reqwest_response<DRSR, T>(
  mut all_params: AllReqParamsMut<'_>,
  client: &Client,
  drsr: &mut DRSR,
  req: T,
) -> crate::Result<reqwest::Response>
where
  T: Send + Serialize<DRSR> + Sync,
{
  if let Some(ref mut rt) = all_params._rt {
    rt.rc.update_params(&rt.rl).await?;
  }
  let mut manage_data = |mut rb: RequestBuilder| {
    if let Some(data_format) = all_params._rp._mime_type {
      rb = rb.header(CONTENT_TYPE, HeaderValue::from_static(data_format._as_str()));
    }
    let mut vec = Vec::with_capacity(128);
    req.to_bytes(&mut vec, drsr)?;
    rb = rb.body(vec);
    crate::Result::Ok(rb)
  };
  let mut rb = match all_params._rp._method {
    Method::Delete => client.delete(all_params._rp._url_parts.url()),
    Method::Get => client.get(all_params._rp._url_parts.url()),
    Method::Patch => manage_data(client.patch(all_params._rp._url_parts.url()))?,
    Method::Post => manage_data(client.post(all_params._rp._url_parts.url()))?,
    Method::Put => manage_data(client.put(all_params._rp._url_parts.url()))?,
  };
  for header in all_params._rp._headers.iter() {
    rb = rb.header(header._key.as_str(), header._value.as_str());
  }
  if let Some(elem) = all_params._rp._user_agent {
    rb = rb.header(USER_AGENT, HeaderValue::from_static(elem._as_str()));
  }
  all_params._reset();
  Ok(rb.send().await?)
}
