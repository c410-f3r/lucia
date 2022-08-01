use crate::{
  api::exchange::ku_coin::{GenericDataResponse, KuCoin},
  data_format::{JsonRequest, JsonResponse},
  network::http::Method,
  types::{MaxAssetAbbr, MaxNumberStr, MaxPairAbbr},
};
use alloc::vec::Vec;

type Res = GenericDataResponse<Vec<V1SymbolsRes>>;

_create_endpoint! {
  KuCoin => JsonResponse|JsonRequest|_json_request;

  V1SymbolsReq<;;>

  |raw: Res, _resp| -> Res { Ok(raw) }

  V1SymbolsParams() -> crate::Result<()> {
    |hp| {
      hp.tp._method = Method::Get;
      hp.tp._url_parts.set_path(format_args!("/api/v1/symbols"))?;
    }
  }

  v1_symbols() {
    || {
      V1SymbolsReq
    }
  }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
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
