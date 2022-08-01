use crate::{
  api::blockchain::solana::{
    Account, AccountEncoding, Commitment, DataSlice, Filter, Solana, SolanaAddressHashStr,
  },
  utils::OneMandAndOneOpt,
};
use alloc::vec::Vec;

_create_json_rpc_endpoint! {
  Solana;

  #[cfg_attr(feature = "serde", serde(transparent))]
  "getProgramAccounts" => GetProgramAccountsReq<
    'bytes: 'filter, 'filter;;S AsRef<str> = &'static str
  >(OneMandAndOneOpt<S, GetProgramAccountsReqParams<'bytes, 'filter>>)

  |raw: Vec<GetProgramAccountsRes>| -> Vec<GetProgramAccountsRes> { raw }

  get_program_accounts(pubkey: S, opt: Option<GetProgramAccountsReqParams<'bytes, 'filter>>) {
    GetProgramAccountsReq(OneMandAndOneOpt(pubkey, opt))
  }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct GetProgramAccountsReqParams<'bytes, 'filter> {
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  pub commitment: Option<Commitment>,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  pub data_slice: Option<DataSlice>,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  pub encoding: Option<AccountEncoding>,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  pub filters: Option<&'filter [Filter<'bytes>]>,
  #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
  pub min_context_slot: Option<i32>,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct GetProgramAccountsRes {
  pub account: Account,
  pub pubkey: SolanaAddressHashStr,
}
