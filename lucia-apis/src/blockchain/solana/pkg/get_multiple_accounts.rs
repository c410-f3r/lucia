#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getMultipleAccounts")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{
    Account, AccountEncoding, Commitment, DataSlice, JsonRpcResponseResultWithContext,
    SolanaHttpPkgsAux,
  };

  #[pkg::aux]
  impl<DRSR> SolanaHttpPkgsAux<DRSR> {}

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[pkg::req_data]
  pub struct GetMultipleAccountsReq<'pks, S>(
    #[pkg::field(name = "pks")] &'pks [S],
    #[pkg::field(name = "config")] Option<GetMultipleAccountsConfig>,
  )
  where
    S: AsRef<str> + Send + Sync;

  #[pkg::res_data]
  pub type GetMultipleAccountsRes = JsonRpcResponseResultWithContext<Vec<Account>>;

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[doc = generic_config_doc!()]
  pub struct GetMultipleAccountsConfig {
    /// Account encoding.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub encoding: Option<AccountEncoding>,
    /// Commitment.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub commitment: Option<Commitment>,
    /// Set the minimum slot that the request can be evaluated at.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub min_context_slot: Option<u64>,
    /// Data slice.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub data_slice: Option<DataSlice>,
  }
}
