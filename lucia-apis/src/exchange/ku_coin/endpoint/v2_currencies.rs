use crate::{
  exchange::ku_coin::{GenericDataResponse, KuCoin},
  misc::{MaxAddressHashStr, _MaxAssetAbbr, _MaxAssetName, _MaxNumberStr},
};
use alloc::boxed::Box;
use arrayvec::ArrayVec;
use lucia::{
  data_formats::{JsonRequest, JsonResponse},
  network::http::Method,
};

type Res = GenericDataResponse<Box<V2CurrenciesRes>>;

_create_endpoint! {
  KuCoin => JsonResponse|JsonRequest|json_request;

  V2CurrenciesReq<;;>

  |raw: Res, _resp| -> Res { Ok(raw) }

  V2CurrenciesParams(asset: &'reqp str) -> crate::Result<()> {
    |hp| {
      hp.tp.method = Method::Get;
      hp.tp.url_parts.set_path(format_args!("/api/v2/currencies/{asset}"))?;
    }
  }

  v2_currencies() {
    || {
      V2CurrenciesReq
    }
  }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct V2CurrenciesRes {
  pub chains: ArrayVec<V2CurrenciesChainRes, 4>,
  pub currency: _MaxAssetAbbr,
  pub name: _MaxAssetName,
  pub full_name: _MaxAssetName,
  pub precision: u8,
  pub confirms: Option<u16>,
  pub contract_address: Option<MaxAddressHashStr>,
  pub is_margin_enabled: bool,
  pub is_debit_enabled: bool,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct V2CurrenciesChainRes {
  pub chain_name: Option<_MaxAssetName>,
  pub confirms: u16,
  pub contract_address: Option<MaxAddressHashStr>,
  pub is_deposit_enabled: bool,
  pub is_withdraw_enabled: bool,
  pub withdrawal_min_fee: _MaxNumberStr,
  pub withdrawal_min_size: _MaxNumberStr,
}
