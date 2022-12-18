#[lucia_macros::pkg(api(KuCoin), data_format(json), error(crate::Error), transport(http))]
pub(crate) mod pkg {
  use crate::{
    exchange::ku_coin::{HttpResWrapper, KuCoin, KuCoinHttpPkgsAux},
    misc::{_MaxNumberStr, into_rslt},
  };
  use lucia::network::HttpReqParams;

  #[pkg::aux]
  impl<DRSR> KuCoinHttpPkgsAux<DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(
    api: &mut KuCoin,
    params: &mut V3GetFullOrderBookParams<'_>,
    req_bytes: &[u8],
    req_params: &mut HttpReqParams,
  ) -> crate::Result<()> {
    api.order_book_rt.rc.update_params(&api.order_book_rt.rl).await?;
    req_params.url.push_path(format_args!("/api/v3/market/orderbook/level2"))?;
    let _ = req_params.url.query_writer()?.write("symbol", params.symbol)?;
    into_rslt(api.credentials.as_mut())?.push_headers(req_bytes, req_params)?;
    Ok(())
  }

  #[derive(Debug)]
  #[pkg::params]
  pub struct V3GetFullOrderBookParams<'any> {
    symbol: &'any str,
  }

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct V3GetFullOrderBookReq;

  #[pkg::res_data]
  pub type V3GetFullOrderBookRes = HttpResWrapper<V3FullOrderBook>;

  #[derive(Debug, serde::Deserialize)]
  #[serde(rename_all = "camelCase")]
  #[doc = _generic_res_data_elem_doc!()]
  pub struct V3FullOrderBook {
    /// Selling offers of base asset.
    pub asks: Vec<(_MaxNumberStr, _MaxNumberStr)>,
    /// Buying offers of base asset.
    pub bids: Vec<(_MaxNumberStr, _MaxNumberStr)>,
    /// KuCoin-specified sequence.
    #[serde(deserialize_with = "crate::misc::_deserialize_from_str")]
    pub sequence: u64,
  }
}
