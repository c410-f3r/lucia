#[lucia_macros::pkg(api(KuCoin), data_format(json), error(crate::Error), transport(http))]
pub(crate) mod pkg {
  use crate::exchange::ku_coin::{HttpResWrapper, KuCoin, KuCoinHttpPkgsAux, V1Ticker};
  use lucia::network::HttpReqParams;

  #[pkg::aux]
  impl<DRSR> KuCoinHttpPkgsAux<DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(
    params: &mut V1GetTickerParams<'_>,
    req_params: &mut HttpReqParams,
  ) -> crate::Result<()> {
    req_params.url.push_path(format_args!("/api/v1/market/orderbook/level1"))?;
    let _ = req_params.url.query_writer()?.write("symbol", params.symbol)?;
    Ok(())
  }

  #[derive(Debug)]
  #[pkg::params]
  pub struct V1GetTickerParams<'any> {
    symbol: &'any str,
  }

  #[derive(Debug, serde::Serialize)]
  #[serde(rename_all = "camelCase")]
  #[pkg::req_data]
  pub struct V1GetTickerReq;

  #[pkg::res_data]
  pub type V1GetTickerRes = HttpResWrapper<V1Ticker>;
}
