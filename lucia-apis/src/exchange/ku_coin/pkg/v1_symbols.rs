#[lucia_macros::pkg(
  api(crate::exchange::ku_coin::KuCoin),
  data_format(json),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::{
    exchange::ku_coin::{GenericDataResponse, KuCoinHttpPackagesAux},
    misc::{_MaxAssetAbbr, _MaxNumberStr, _MaxPairAbbr},
  };
  use lucia::network::HttpReqParams;

  #[pkg::aux]
  impl<DRSR> KuCoinHttpPackagesAux<DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(req_params: &mut HttpReqParams) -> crate::Result<()> {
    req_params.url.push_path(format_args!("/api/v1/symbols"))?;
    Ok(())
  }

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[pkg::req_data]
  pub struct V1SymbolsReqData;

  #[pkg::res_data]
  pub type V1SymbolsResData = GenericDataResponse<Vec<V1SymbolsElemResData>>;

  #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
  #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  pub struct V1SymbolsElemResData {
    pub symbol: _MaxPairAbbr,
    pub name: _MaxPairAbbr,
    pub base_currency: _MaxAssetAbbr,
    pub quote_currency: _MaxAssetAbbr,
    pub fee_currency: _MaxAssetAbbr,
    pub market: _MaxAssetAbbr,
    pub base_min_size: _MaxNumberStr,
    pub quote_min_size: _MaxNumberStr,
    pub base_max_size: _MaxNumberStr,
    pub quote_max_size: _MaxNumberStr,
    pub base_increment: _MaxNumberStr,
    pub quote_increment: _MaxNumberStr,
    pub price_increment: _MaxNumberStr,
    pub price_limit_rate: _MaxNumberStr,
    pub is_margin_enabled: bool,
    pub enable_trading: bool,
  }
}
