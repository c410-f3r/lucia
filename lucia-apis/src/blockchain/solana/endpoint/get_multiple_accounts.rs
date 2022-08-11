use crate::{
  blockchain::solana::{
    Account, AccountEncoding, Commitment, DataSlice, JsonRpcResponseResultWithContext, Solana,
  },
  misc::OneMandAndOneOpt,
};
use alloc::vec::Vec;

type Res = JsonRpcResponseResultWithContext<Vec<Account>>;

_create_json_rpc_endpoint! {
  Solana;

  #[cfg_attr(feature = "serde", serde(transparent))]
  "getMultipleAccounts" => GetMultipleAccountsReq<'addrs;;S AsRef<str> = &'static str : 'addrs>(
    OneMandAndOneOpt<&'addrs [S], GetMultipleAccountsReqOptParams>
  )

  |raw: Res| -> Res { raw }

  get_multiple_accounts(addrs: &'addrs [S], opt: Option<GetMultipleAccountsReqOptParams>) {
    GetMultipleAccountsReq(OneMandAndOneOpt(addrs, opt))
  }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct GetMultipleAccountsReqOptParams {
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  pub commitment: Option<Commitment>,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  pub data_slice: Option<DataSlice>,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  pub encoding: Option<AccountEncoding>,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  pub min_context_slot: Option<i32>,
}
