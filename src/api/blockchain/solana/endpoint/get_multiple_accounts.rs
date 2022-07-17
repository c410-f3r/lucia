use crate::{
  api::blockchain::solana::{
    Account, AccountEncoding, Commitment, DataSlice, JsonRpcResponseResultWithContext, Solana,
  },
  utils::OneMandAndOneOpt,
};
use alloc::vec::Vec;

type Res = JsonRpcResponseResultWithContext<Vec<Account>>;

_create_json_rpc_endpoint! {
  Solana;

  #[serde(transparent)]
  "getMultipleAccounts" => GetMultipleAccountsReq<'addrs;;S AsRef<str> = &'static str : 'addrs>(
    OneMandAndOneOpt<&'addrs [S], GetMultipleAccountsReqOptParams>
  )

  |raw: Res| -> Res { raw }

  get_multiple_accounts(addrs: &'addrs [S], opt: Option<GetMultipleAccountsReqOptParams>) {
    GetMultipleAccountsReq(OneMandAndOneOpt(addrs, opt))
  }
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMultipleAccountsReqOptParams {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub commitment: Option<Commitment>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub data_slice: Option<DataSlice>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub encoding: Option<AccountEncoding>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub min_context_slot: Option<i32>,
}
