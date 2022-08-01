use crate::{
  api::exchange::ku_coin::{GenericDataResponse, KuCoin},
  data_format::{JsonRequest, JsonResponse},
  network::http::Method,
  types::{MaxAddressHashStr, MaxAssetAbbr, MaxAssetName, MaxNumberStr},
};
use alloc::{string::String, vec::Vec};

type Res = GenericDataResponse<Vec<V1CurrenciesRes>>;

_create_endpoint! {
  KuCoin => JsonResponse|JsonRequest|_json_request;

  V1CurrenciesReq<;;>

  |raw: Res, _resp| -> Res { Ok(raw) }

  V1CurrenciesParams() -> crate::Result<()> {
    |hp| {
      hp.tp._method = Method::Get;
      hp.tp._url_parts.set_path(format_args!("/api/v1/currencies"))?;
    }
  }

  v1_currencies() {
    || {
      V1CurrenciesReq
    }
  }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct V1CurrenciesRes {
  pub confirms: Option<u16>,
  pub contract_address: Option<MaxAddressHashStr>,
  pub currency: MaxAssetAbbr,
  pub full_name: String,
  pub is_debit_enabled: bool,
  pub is_deposit_enabled: Option<bool>,
  pub is_margin_enabled: bool,
  pub is_withdraw_enabled: Option<bool>,
  pub name: MaxAssetName,
  pub precision: u8,
  pub withdrawal_min_fee: Option<MaxNumberStr>,
  pub withdrawal_min_size: Option<MaxNumberStr>,
}
