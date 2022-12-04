#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getTokenAccountsByOwner")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{
    Account, AccountEncoding, Commitment, DataSlice, JsonRpcResponseResultWithContext,
    MintOrProgramId, SolanaAddressHashStr, SolanaHttpPackagesAux,
  };

  #[pkg::aux]
  impl<DRSR> SolanaHttpPackagesAux<DRSR> {}

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[pkg::req_data]
  pub struct GetTokenAccountsByOwnerReq<S>(
    #[pkg::field(name = "pk")] S,
    #[pkg::field(name = "criteria")] MintOrProgramId<S>,
    #[pkg::field(name = "config")] Option<GetTokenAccountsByOwnerConfig>,
  )
  where
    S: AsRef<str> + Send;

  #[pkg::res_data]
  pub type GetTokenAccountsByOwnerRes =
    JsonRpcResponseResultWithContext<Vec<GetTokenAccountsByOwnerResElem>>;

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[doc = generic_config_doc!()]
  pub struct GetTokenAccountsByOwnerConfig {
    /// Account encoding
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub encoding: Option<AccountEncoding>,
    /// Commitment
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub commitment: Option<Commitment>,
    /// Set the minimum slot that the request can be evaluated at.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub min_context_slot: Option<u64>,
    /// Data slice
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub data_slice: Option<DataSlice>,
  }

  #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
  #[derive(Debug)]
  #[doc = _generic_res_data_elem_doc!()]
  pub struct GetTokenAccountsByOwnerResElem {
    /// Account
    pub account: Account,
    /// Base58 identifier.
    pub pubkey: SolanaAddressHashStr,
  }
}
