#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getBalance")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{
    Commitment, DataSlice, JsonRpcResponseResultWithContext, SolanaHttpPkgsAux,
  };
  use lucia::misc::AsyncTrait;

  #[pkg::aux]
  impl<DRSR> SolanaHttpPkgsAux<DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetBalanceReq<S>(
    #[pkg::field(name = "pk")] S,
    #[pkg::field(name = "config")]
    #[serde(skip_serializing_if = "Option::is_none")]
    Option<GetBalanceConfig>,
  )
  where
    S: AsyncTrait;

  #[pkg::res_data]
  pub type GetBalanceRes = JsonRpcResponseResultWithContext<u64>;

  #[derive(Debug, serde::Serialize)]
  #[doc = generic_config_doc!()]
  pub struct GetBalanceConfig {
    #[doc = commitment_doc!()]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    /// Data slice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_slice: Option<DataSlice>,
  }
}
