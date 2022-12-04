#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getBalance")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{
    Commitment, DataSlice, JsonRpcResponseResultWithContext, SolanaHttpPackagesAux,
  };

  #[pkg::aux]
  impl<DRSR> SolanaHttpPackagesAux<DRSR> {}

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[pkg::req_data]
  pub struct GetBalanceReq<S>(
    #[pkg::field(name = "pk")] S,
    #[pkg::field(name = "config")] Option<GetBalanceConfig>,
  )
  where
    S: AsRef<str> + Send;

  #[pkg::res_data]
  pub type GetBalanceRes = JsonRpcResponseResultWithContext<u64>;

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[doc = generic_config_doc!()]
  pub struct GetBalanceConfig {
    /// Commitment.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub commitment: Option<Commitment>,
    /// Data slice.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub data_slice: Option<DataSlice>,
  }
}
