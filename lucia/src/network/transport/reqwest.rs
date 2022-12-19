use crate::{
  misc::manage_before_sending_related,
  network::{http::HttpMethod, transport::Transport, HttpParams, TransportGroup},
  pkg::{Package, PkgsAux},
};
#[cfg(feature = "async-trait")]
use alloc::boxed::Box;
use reqwest::{
  header::{HeaderValue, CONTENT_TYPE, USER_AGENT},
  Client, RequestBuilder,
};

/// ```rust,no_run
/// # async fn fun() -> lucia::Result<()> {
/// use lucia::{
///   network::{transport::Transport, HttpParams},
///   pkg::PkgsAux,
/// };
/// let _ = reqwest::Client::new()
///   .send_retrieve_and_decode_contained(
///     &mut (),
///     &mut PkgsAux::from_minimum((), (), HttpParams::from_url("URL")?.into()),
///   )
///   .await?;
/// # Ok(()) }
/// ```
#[cfg_attr(feature = "async-trait", async_trait::async_trait)]
impl<DRSR> Transport<DRSR> for Client
where
  DRSR: Send + Sync,
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
    P: Package<DRSR, HttpParams> + Send + Sync,
  {
    let _res = response(self, pkg, pkgs_aux).await?;
    Ok(())
  }

  #[inline]
  async fn send_and_retrieve<P>(
    &mut self,
    pkg: &mut P,
    pkgs_aux: &mut PkgsAux<P::Api, DRSR, Self::Params>,
  ) -> Result<usize, P::Error>
  where
    P: Package<DRSR, HttpParams> + Send + Sync,
  {
    let res = response(self, pkg, pkgs_aux).await?;
    let received_bytes = res.bytes().await.map_err(Into::into)?;
    pkgs_aux.byte_buffer.extend(received_bytes.into_iter());
    Ok(pkgs_aux.byte_buffer.len())
  }
}

#[inline]
async fn response<DRSR, P>(
  client: &Client,
  pkg: &mut P,
  pkgs_aux: &mut PkgsAux<P::Api, DRSR, HttpParams>,
) -> Result<reqwest::Response, P::Error>
where
  DRSR: Send + Sync,
  P: Package<DRSR, HttpParams> + Send + Sync,
{
  fn manage_data<A, DRSR>(
    mut rb: RequestBuilder,
    pkgs_aux: &mut PkgsAux<A, DRSR, HttpParams>,
  ) -> RequestBuilder
  where
    DRSR: Send + Sync,
  {
    if let Some(data_format) = pkgs_aux.ext_req_params.mime_type {
      rb = rb.header(CONTENT_TYPE, HeaderValue::from_static(data_format._as_str()));
    }
    rb = rb.body(pkgs_aux.byte_buffer.clone());
    rb
  }
  pkgs_aux.byte_buffer.clear();
  manage_before_sending_related(pkg, pkgs_aux, client).await?;
  let mut rb = match pkgs_aux.ext_req_params.method {
    HttpMethod::Delete => client.delete(pkgs_aux.ext_req_params.url.url()),
    HttpMethod::Get => client.get(pkgs_aux.ext_req_params.url.url()),
    HttpMethod::Patch => manage_data(client.patch(pkgs_aux.ext_req_params.url.url()), pkgs_aux),
    HttpMethod::Post => manage_data(client.post(pkgs_aux.ext_req_params.url.url()), pkgs_aux),
    HttpMethod::Put => manage_data(client.put(pkgs_aux.ext_req_params.url.url()), pkgs_aux),
  };
  pkgs_aux.byte_buffer.clear();
  for (key, value) in pkgs_aux.ext_req_params.headers.iter() {
    rb = rb.header(key, value);
  }
  if let Some(elem) = pkgs_aux.ext_req_params.user_agent {
    rb = rb.header(USER_AGENT, HeaderValue::from_static(elem._as_str()));
  }
  pkgs_aux.ext_req_params.url.retain_origin()?;
  let res = rb.send().await.map_err(Into::into)?;
  pkgs_aux.ext_res_params.status_code = <_>::try_from(Into::<u16>::into(res.status()))?;
  pkg.after_sending(&mut pkgs_aux.api, &mut pkgs_aux.ext_res_params).await?;
  pkgs_aux.ext_req_params.headers.clear();
  Ok(res)
}
