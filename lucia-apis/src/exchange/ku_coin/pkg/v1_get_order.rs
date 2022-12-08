#[lucia_macros::pkg(api(KuCoin), data_format(json), error(crate::Error), transport(http))]
pub(crate) mod pkg {
  use crate::{
    exchange::ku_coin::{HttpResWrapper, KuCoin, KuCoinHttpPkgsAux, V1Order},
    misc::into_rslt,
  };
  use lucia::network::HttpReqParams;

  #[pkg::aux]
  impl<DRSR> KuCoinHttpPkgsAux<DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(
    api: &mut KuCoin,
    params: &mut V1GetOrderParams<'_>,
    req_bytes: &[u8],
    req_params: &mut HttpReqParams,
  ) -> crate::Result<()> {
    api.orders_rt.rc.update_params(&api.orders_rt.rl).await?;
    req_params.url.push_path(format_args!("/api/v1/orders/{}", params.order_id))?;
    into_rslt(api.credentials.as_mut())?.push_headers(req_bytes, req_params)?;
    Ok(())
  }

  #[derive(Debug)]
  #[pkg::params]
  pub struct V1GetOrderParams<'any> {
    order_id: &'any str,
  }

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
  #[derive(Debug)]
  #[pkg::req_data]
  pub struct V1GetOrderReq;

  #[pkg::res_data]
  pub type V1GetOrderRes = HttpResWrapper<Box<V1Order>>;
}
