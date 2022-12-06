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

  #[pkg::aux]
  impl<DRSR> SolanaHttpPkgsAux<DRSR> {}

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[pkg::req_data]
  pub struct GetProgramAccountsReq<'bytes, 'filter, S>(
    #[pkg::field(name = "pk")] S,
    #[pkg::field(name = "config")] Option<GetProgramAccountsConfig<'bytes, 'filter>>,
  )
  where
    S: AsRef<str> + Send;

  #[pkg::res_data]
  pub type GetProgramAccountsRes = Vec<GetProgramAccountsResElem>;

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[doc = generic_config_doc!()]
  pub struct GetProgramAccountsConfig<'bytes, 'filter> {
    /// Account encoding
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub encoding: Option<AccountEncoding>,
    /// Commitment
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub commitment: Option<Commitment>,
    /// Filters
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub filters: Option<&'filter [Filter<'bytes>]>,
    /// Minimum slot at which to perform preflight transaction check
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub min_context_slot: Option<u64>,
    /// Data slice
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub data_slice: Option<DataSlice>,
  }

  #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
  #[derive(Debug)]
  #[doc = _generic_res_data_elem_doc!()]
  pub struct GetProgramAccountsResElem {
    /// Account
    pub account: Account,
    /// Base58 identifier
    pub pubkey: ArrayString<44>,
  }
}
