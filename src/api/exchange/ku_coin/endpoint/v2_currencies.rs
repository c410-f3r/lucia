use crate::{
  api::exchange::ku_coin::{endpoint::GenericDataResponse, KuCoin},
  network::HttpMethod,
  types::{MaxAddressHashStr, MaxAssetAbbr, MaxAssetName, MaxNumberStr},
};
use arrayvec::ArrayVec;
use core::fmt::Write;

type Res = GenericDataResponse<V2CurrenciesRes>;

_create_json_endpoint! {
  KuCoin;

  V2CurrenciesReq<;;>

  |raw: Res| -> Res { Ok(raw) }

  v2_currencies(asset: &str) -> crate::Result<:> {
    |api, tp| {
      tp.http_params._set(HttpMethod::Get, api.urls.v2_currencies.url());
      tp.http_params.url.write_fmt(format_args!("/{asset}"))?;
      V2CurrenciesReq
    }
  }

  Ok
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
