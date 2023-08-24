use crate::{
  misc::{manage_after_sending_related, manage_before_sending_related, AsyncTrait},
  network::{
    http::HttpMethod,
    transport::{Transport, TransportParams},
    HttpParams, TransportGroup,
  },
  pkg::{Package, PkgsAux},
};
use core::ops::Range;
use surf::{
  http::headers::{CONTENT_TYPE, USER_AGENT},
  Client, RequestBuilder,
};

/// ```rust,no_run
/// # async fn fun() -> lucia::Result<()> {
/// use lucia::{
///   network::{transport::Transport, HttpParams},
///   pkg::PkgsAux,
/// };
/// let _ = surf::Client::new()
///   .send_retrieve_and_decode_contained(
///     &mut (),
///     &mut PkgsAux::from_minimum((), (), HttpParams::from_url("URL")?),
///   )
///   .await?;
/// # Ok(()) }
/// ```
#[cfg_attr(feature = "async-trait", async_trait::async_trait)]
impl<DRSR> Transport<DRSR> for Client
where
  DRSR: AsyncTrait,
{
  const GROUP: TransportGroup = TransportGroup::HTTP;
  type Params = HttpParams;

  #[inline]
  async fn send<P>(
    &mut self,
    pkg: &mut P,
    pkgs_aux: &mut PkgsAux<P::Api, DRSR, Self::Params>,
  ) -> Result<(), P::Error>
  where
    P: Package<DRSR, HttpParams>,
  {
    let _res = response(self, pkg, pkgs_aux).await?;
    Ok(())
  }

  #[inline]
  async fn send_and_retrieve<P>(
    &mut self,
    pkg: &mut P,
    pkgs_aux: &mut PkgsAux<P::Api, DRSR, Self::Params>,
  ) -> Result<Range<usize>, P::Error>
  where
    P: Package<DRSR, HttpParams>,
  {
    let mut res = response(self, pkg, pkgs_aux).await?;
    let received_bytes = res.body_bytes().await.map_err(Into::into)?;
    pkgs_aux.byte_buffer.extend(received_bytes.into_iter());
    Ok(0..pkgs_aux.byte_buffer.len())
  }
}

async fn response<DRSR, P>(
  client: &Client,
  pkg: &mut P,
  pkgs_aux: &mut PkgsAux<P::Api, DRSR, HttpParams>,
) -> Result<surf::Response, P::Error>
where
  DRSR: AsyncTrait,
  P: Package<DRSR, HttpParams>,
{
  async fn manage_data<API, E, DRSR>(
    mut rb: RequestBuilder,
    pkgs_aux: &mut PkgsAux<API, DRSR, HttpParams>,
  ) -> Result<RequestBuilder, E> {
    if let Some(data_format) = &pkgs_aux.tp.ext_req_params().mime_type {
      rb = rb.header(CONTENT_TYPE, data_format._as_str());
    }
    rb = rb.body(&*pkgs_aux.byte_buffer);
    Ok(rb)
  }
  pkgs_aux.byte_buffer.clear();
  manage_before_sending_related(pkg, pkgs_aux, client).await?;
  let mut rb = match pkgs_aux.tp.ext_req_params().method {
    HttpMethod::Delete => client.delete(pkgs_aux.tp.ext_req_params().url.url()),
    HttpMethod::Get => client.get(pkgs_aux.tp.ext_req_params().url.url()),
    HttpMethod::Patch => {
      manage_data(client.patch(pkgs_aux.tp.ext_req_params().url.url()), pkgs_aux).await?
    }
    HttpMethod::Post => {
      manage_data(client.post(pkgs_aux.tp.ext_req_params().url.url()), pkgs_aux).await?
    }
    HttpMethod::Put => {
      manage_data(client.put(pkgs_aux.tp.ext_req_params().url.url()), pkgs_aux).await?
    }
  };
  pkgs_aux.byte_buffer.clear();
  for (key, value) in pkgs_aux.tp.ext_req_params().headers.iter() {
    rb = rb.header(key, value);
  }
  if let Some(elem) = pkgs_aux.tp.ext_req_params().user_agent {
    rb = rb.header(USER_AGENT, elem._as_str());
  }
  let res = rb.await.map_err(Into::into)?;
  for (key, value) in res.iter() {
    pkgs_aux.tp.ext_res_params_mut().headers.push_str(key.as_str(), value.as_str())?;
  }
  pkgs_aux.tp.ext_res_params_mut().status_code = <_>::try_from(Into::<u16>::into(res.status()))?;
  manage_after_sending_related(pkg, pkgs_aux, client).await?;
  pkgs_aux.tp.reset();
  Ok(res)
}
