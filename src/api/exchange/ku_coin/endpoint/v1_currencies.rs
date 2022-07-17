use crate::{
  api::exchange::ku_coin::{GenericDataResponse, KuCoin},
  network::HttpMethod,
  types::{MaxAddressHashStr, MaxAssetAbbr, MaxAssetName, MaxNumberStr},
};
use alloc::{string::String, vec::Vec};

type Res = GenericDataResponse<Vec<V1CurrenciesRes>>;

_create_json_endpoint! {
  KuCoin;

  V1CurrenciesReq<;;>

  |raw: Res| -> Res { Ok(raw) }

  V1CurrenciesParams() -> crate::Result<()> {
    |hp| {
      hp._method = HttpMethod::_Get;
      hp._url_parts.set_path(format_args!("/api/v1/currencies"))?;
    }
  }

  v1_currencies() {
    || {
      V1CurrenciesReq
    }
  }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
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
