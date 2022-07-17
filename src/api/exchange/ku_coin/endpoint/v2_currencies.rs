use crate::{
  api::exchange::ku_coin::{GenericDataResponse, KuCoin},
  network::HttpMethod,
  types::{MaxAddressHashStr, MaxAssetAbbr, MaxAssetName, MaxNumberStr},
};
use arrayvec::ArrayVec;

type Res = GenericDataResponse<V2CurrenciesRes>;

_create_json_endpoint! {
  KuCoin;

  V2CurrenciesReq<;;>

  |raw: Res| -> Res { Ok(raw) }

  V2CurrenciesParams(asset: &'rpd str) -> crate::Result<()> {
    |hp| {
      hp._method = HttpMethod::_Get;
      hp._url_parts.set_path(format_args!("/api/v2/currencies/{asset}"))?;
    }
  }

  v2_currencies() {
    || {
      V2CurrenciesReq
    }
  }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V2CurrenciesRes {
  pub chains: ArrayVec<V2CurrenciesChainRes, 4>,
  pub currency: MaxAssetAbbr,
  pub name: MaxAssetName,
  pub full_name: MaxAssetName,
  pub precision: u8,
  pub confirms: Option<u16>,
  pub contract_address: Option<MaxAddressHashStr>,
  pub is_margin_enabled: bool,
  pub is_debit_enabled: bool,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V2CurrenciesChainRes {
  pub chain_name: Option<MaxAssetName>,
  pub confirms: u16,
  pub contract_address: Option<MaxAddressHashStr>,
  pub is_deposit_enabled: bool,
  pub is_withdraw_enabled: bool,
  pub withdrawal_min_fee: MaxNumberStr,
  pub withdrawal_min_size: MaxNumberStr,
}
