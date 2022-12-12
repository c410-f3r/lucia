#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getMinimumBalanceForRentExemption")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{Commitment, SolanaHttpPkgsAux};

  #[pkg::aux]
  impl<DRSR> SolanaHttpPkgsAux<DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetMinimumBalanceForRentExemptionReq(
    #[pkg::field(name = "data_len")] usize,
    #[pkg::field(name = "config")] Option<GetMinimumBalanceForRentExemptionConfig>,
  );

  #[pkg::res_data]
  pub type GetMinimumBalanceForRentExemptionRes = u64;

  #[derive(Debug, serde::Serialize)]
  #[doc = generic_config_doc!()]
  pub struct GetMinimumBalanceForRentExemptionConfig {
    /// Commitment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
  }
}
