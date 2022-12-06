use crate::{
  misc::manage_before_sending_related,
  network::{http::HttpMethod, transport::Transport, HttpParams, TransportGroup},
  package::{Package, PackagesAux},
};
#[cfg(not(feature = "async-fn-in-trait"))]
use alloc::boxed::Box;
use surf::{
  http::headers::{CONTENT_TYPE, USER_AGENT},
  Client, RequestBuilder,
};

/// ```rust,no_run
/// # async fn fun() -> lucia::Result<()> {
/// use lucia::{
///   network::{transport::Transport, HttpParams},
///   package::PackagesAux,
/// };
/// let _ = surf::Client::new()
///   .send_retrieve_and_decode_contained(
///     &mut (),
///     &mut PackagesAux::from_minimum((), (), HttpParams::from_url("URL")?),
///   )
///   .await?;
/// # Ok(()) }
/// ```
#[cfg_attr(not(feature = "async-fn-in-trait"), async_trait::async_trait)]
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
    pkgs_aux: &mut PackagesAux<P::Api, DRSR, Self::Params>,
  ) -> Result<(), P::Error>
  where
    DRSR: Send + Sync,
    P: Package<DRSR, HttpParams> + Send + Sync,
    P::Api: Send + Sync,
  {
    let _ = response(self, pkg, pkgs_aux).await?;
    Ok(())
  }

  #[inline]
  async fn send_and_retrieve<P>(
    &mut self,
    pkg: &mut P,
    pkgs_aux: &mut PackagesAux<P::Api, DRSR, Self::Params>,
  ) -> Result<usize, P::Error>
  where
    P: Package<DRSR, HttpParams> + Send + Sync,
  {
    let mut res = response(self, pkg, pkgs_aux).await?;
    let received_bytes = res.body_bytes().await.map_err(Into::into)?;
    pkgs_aux.byte_buffer.extend(received_bytes.into_iter());
    Ok(pkgs_aux.byte_buffer.len())
  }
}

#[inline]
async fn response<DRSR, P>(
  client: &Client,
  pkg: &mut P,
  pkgs_aux: &mut PackagesAux<P::Api, DRSR, HttpParams>,
) -> Result<surf::Response, P::Error>
where
  DRSR: Send + Sync,
  P: Package<DRSR, HttpParams> + Send + Sync,
{
  async fn manage_data<A, E, DRSR>(
    mut rb: RequestBuilder,
    pkgs_aux: &mut PackagesAux<A, DRSR, HttpParams>,
  ) -> Result<RequestBuilder, E>
  where
    DRSR: Send + Sync,
  {
    if let Some(data_format) = pkgs_aux.ext_req_params.mime_type {
      rb = rb.header(CONTENT_TYPE, data_format._as_str());
    }
    rb = rb.body(&*pkgs_aux.byte_buffer);
    Ok(rb)
  }
  pkgs_aux.byte_buffer.clear();
  manage_before_sending_related(pkg, pkgs_aux, client).await?;
  let mut rb = match pkgs_aux.ext_req_params.method {
    HttpMethod::Delete => client.delete(pkgs_aux.ext_req_params.url.url()),
    HttpMethod::Get => client.get(pkgs_aux.ext_req_params.url.url()),
    HttpMethod::Patch => {
      manage_data(client.patch(pkgs_aux.ext_req_params.url.url()), pkgs_aux).await?
    }
    HttpMethod::Post => {
      manage_data(client.post(pkgs_aux.ext_req_params.url.url()), pkgs_aux).await?
    }
    HttpMethod::Put => manage_data(client.put(pkgs_aux.ext_req_params.url.url()), pkgs_aux).await?,
  };
  pkgs_aux.byte_buffer.clear();
  for (key, value) in pkgs_aux.ext_req_params.headers.iter() {
    rb = rb.header(key, value);
  }
  if let Some(elem) = pkgs_aux.ext_req_params.user_agent {
    rb = rb.header(USER_AGENT, elem._as_str());
  }
  pkgs_aux.ext_req_params.url.retain_origin()?;
  let res = rb.await.map_err(Into::into)?;
  pkgs_aux.ext_res_params.status_code = <_>::try_from(Into::<u16>::into(res.status()))?;
  pkg.after_sending(&mut pkgs_aux.api, &mut pkgs_aux.ext_res_params).await?;
  pkgs_aux.ext_req_params.headers.clear();
  Ok(res)
}
