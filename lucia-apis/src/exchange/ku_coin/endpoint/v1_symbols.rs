use crate::{
  exchange::ku_coin::{GenericDataResponse, KuCoin},
  misc::{_MaxAssetAbbr, _MaxNumberStr, _MaxPairAbbr},
};
use alloc::vec::Vec;
use lucia::{
  data_formats::{JsonRequest, JsonResponse},
  network::http::Method,
};

type Res = GenericDataResponse<Vec<V1SymbolsRes>>;

_create_endpoint! {
  KuCoin => JsonResponse|JsonRequest|json_request;

  V1SymbolsReq<;;>

  |raw: Res, _resp| -> Res { Ok(raw) }

  V1SymbolsParams() -> crate::Result<()> {
    |hp| {
      hp.tp.method = Method::Get;
      hp.tp.url_parts.set_path(format_args!("/api/v1/symbols"))?;
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
