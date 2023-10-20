#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getProgramAccounts")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{
    Account, AccountEncoding, Commitment, DataSlice, Filter, SolanaHttpPkgsAux,
  };
  use arrayvec::ArrayString;
  use lucia::misc::AsyncBounds;

  #[pkg::aux]
  impl<DRSR> SolanaHttpPkgsAux<DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetProgramAccountsReq<'bytes, 'filter, S>(
    #[pkg::field(name = "pk")] S,
    #[pkg::field(name = "config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    Option<GetProgramAccountsConfig<'bytes, 'filter>>,
  )
  where
    S: AsyncBounds;

  #[pkg::res_data]
  pub type GetProgramAccountsRes = Vec<GetProgramAccounts>;

  #[derive(Debug, serde::Serialize)]
  #[doc = generic_config_doc!()]
  #[serde(rename_all = "camelCase")]
  pub struct GetProgramAccountsConfig<'bytes, 'filter> {
    /// Account encoding
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<AccountEncoding>,
    /// Commitment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    /// Filters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<&'filter [Filter<'bytes>]>,
    #[doc = min_context_slot_doc!()]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_context_slot: Option<u64>,
    /// Data slice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_slice: Option<DataSlice>,
  }

  #[derive(Debug, serde::Deserialize)]
  #[doc = _generic_res_data_elem_doc!()]
  pub struct GetProgramAccounts {
    /// Account
    pub account: Account,
    /// Base58 identifier
    pub pubkey: ArrayString<44>,
  }
}
