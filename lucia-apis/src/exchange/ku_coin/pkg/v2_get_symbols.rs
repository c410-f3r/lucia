#[lucia_macros::pkg(
  api(crate::exchange::ku_coin::KuCoin),
  data_format(json),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::{
    exchange::ku_coin::{HttpResWrapper, KuCoinHttpPkgsAux},
    misc::{_MaxAssetAbbr, _MaxNumberStr, _MaxPairAbbr},
  };
  use lucia::network::HttpReqParams;

  #[pkg::aux]
  impl<DRSR> KuCoinHttpPkgsAux<DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(
    params: &mut V2GetSymbolsParams<'_>,
    req_params: &mut HttpReqParams,
  ) -> crate::Result<()> {
    req_params.url.push_path(format_args!("/api/v2/symbols"))?;
    let _ = req_params.url.query_writer()?.write_opt("market", params.market)?;
    Ok(())
  }

  #[derive(Debug)]
  #[pkg::params]
  pub struct V2GetSymbolsParams<'any> {
    market: Option<&'any str>,
  }

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[pkg::req_data]
  pub struct V2GetSymbolsReq;

  #[pkg::res_data]
  pub type V2GetSymbolsRes = HttpResWrapper<Vec<V2Symbol>>;

  #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
  #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
  #[derive(Debug)]
  #[doc = _generic_res_data_elem_doc!()]
  pub struct V2Symbol {
    /// Left-hand-side asset.
    pub base_currency: _MaxAssetAbbr,
    /// Order increment of the left-hand-side asset.
    pub base_increment: _MaxNumberStr,
    /// Maximum order size of the left-hand-side asset.
    pub base_max_size: _MaxNumberStr,
    /// Minimum order size  of the left-hand-side asset.
    pub base_min_size: _MaxNumberStr,
    /// Pair is or is available for trading.
    pub enable_trading: bool,
    /// Charged fees in the given asset.
    pub fee_currency: _MaxAssetAbbr,
    /// Pair is or is available for margin markets.
    pub is_margin_enabled: bool,
    /// Trading market.
    pub market: _MaxAssetAbbr,
    /// Minimum spot and margin trading amounts.
    pub min_funds: Option<_MaxNumberStr>,
    /// Mutable pair of assets.
    pub name: _MaxPairAbbr,
    /// The increment of the price required to place a limit order.
    pub price_increment: _MaxNumberStr,
    /// Threshold for price protection.
    pub price_limit_rate: _MaxNumberStr,
    /// Right-hand-side asset.
    pub quote_currency: _MaxAssetAbbr,
    /// Order increment of the right-hand-side asset.
    pub quote_increment: _MaxNumberStr,
    /// Maximum order size of the right-hand-side asset.
    pub quote_max_size: _MaxNumberStr,
    /// Minimum order size of the right-hand-side asset.
    pub quote_min_size: _MaxNumberStr,
    /// Immutable pair of assets.
    pub symbol: _MaxPairAbbr,
  }
}
