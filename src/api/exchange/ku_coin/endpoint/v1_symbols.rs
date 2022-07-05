use crate::{
  api::exchange::ku_coin::{endpoint::GenericDataResponse, KuCoin},
  network::HttpMethod,
  types::{MaxAssetAbbr, MaxNumberStr, MaxPairAbbr},
};
use alloc::vec::Vec;

type Res = GenericDataResponse<Vec<V1SymbolsRes>>;

_create_json_endpoint! {
  KuCoin;

  V1SymbolsReq<;;>

  |raw: Res| -> Res { Ok(raw) }

  v1_symbols() {
    |api, tp| {
      tp._http_params._set(HttpMethod::Get, None, api.urls.v1_symbols.url());
      V1SymbolsReq
    }
  }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V1SymbolsRes {
  pub symbol: MaxPairAbbr,
  pub name: MaxPairAbbr,
  pub base_currency: MaxAssetAbbr,
  pub quote_currency: MaxAssetAbbr,
  pub fee_currency: MaxAssetAbbr,
  pub market: MaxAssetAbbr,
  pub base_min_size: MaxNumberStr,
  pub quote_min_size: MaxNumberStr,
  pub base_max_size: MaxNumberStr,
  pub quote_max_size: MaxNumberStr,
  pub base_increment: MaxNumberStr,
  pub quote_increment: MaxNumberStr,
  pub price_increment: MaxNumberStr,
  pub price_limit_rate: MaxNumberStr,
  pub is_margin_enabled: bool,
  pub enable_trading: bool,
}
