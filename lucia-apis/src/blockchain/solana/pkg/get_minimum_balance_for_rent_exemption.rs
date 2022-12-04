#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getMinimumBalanceForRentExemption")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{Commitment, SolanaHttpPackagesAux};

  #[pkg::aux]
  impl<DRSR> SolanaHttpPackagesAux<DRSR> {}

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[pkg::req_data]
  pub struct GetMinimumBalanceForRentExemptionReq(
    #[pkg::field(name = "data_len")] usize,
    #[pkg::field(name = "config")] Option<GetMinimumBalanceForRentExemptionConfig>,
  );

  #[pkg::res_data]
  pub type GetMinimumBalanceForRentExemptionRes = u64;

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[doc = generic_config_doc!()]
  pub struct GetMinimumBalanceForRentExemptionConfig {
    /// Commitment
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub commitment: Option<Commitment>,
  }
}
