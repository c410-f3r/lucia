use crate::{
  api::blockchain::solana::{
    endpoint::{Commitment, DataSlice},
    Account, AccountEncoding, Filter, Solana, SolanaAddressHashStr,
  },
  utils::OneMandAndOneOpt,
};
use alloc::vec::Vec;

_create_json_rpc_endpoint! {
  Solana;

  #[serde(transparent)]
  "getProgramAccounts" => GetProgramAccountsReq<
    'bytes: 'filter, 'filter;;S AsRef<str> = &'static str
  >(OneMandAndOneOpt<S, GetProgramAccountsReqParams<'bytes, 'filter>>)

  |raw: Vec<GetProgramAccountsRes>| -> Vec<GetProgramAccountsRes> { raw }

  get_program_accounts(pubkey: S, opt: Option<GetProgramAccountsReqParams<'bytes, 'filter>>) {
    GetProgramAccountsReq(OneMandAndOneOpt(pubkey, opt))
  }
}

#[derive(Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetProgramAccountsReqParams<'bytes, 'filter> {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub commitment: Option<Commitment>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub data_slice: Option<DataSlice>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub encoding: Option<AccountEncoding>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub filters: Option<&'filter [Filter<'bytes>]>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub min_context_slot: Option<i32>,
}

#[derive(Debug, Eq, PartialEq, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetProgramAccountsRes {
  pub account: Account,
  pub pubkey: SolanaAddressHashStr,
}
