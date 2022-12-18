#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getBlocksWithLimit")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{Commitment, SolanaHttpPkgsAux};

  #[pkg::aux]
  impl<DRSR> SolanaHttpPkgsAux<DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetBlocksWithLimitReq(
    #[pkg::field(name = "start_slot")] u64,
    #[pkg::field(name = "limit")] Option<u64>,
    #[pkg::field(name = "config")] Option<GetBlocksWithLimitConfig>,
  );

  #[pkg::res_data]
  pub type GetBlocksWithLimitRes = Vec<u64>;

  #[derive(Debug, serde::Serialize)]
  #[doc = generic_config_doc!()]
  pub struct GetBlocksWithLimitConfig {
    #[doc = commitment_doc!()]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
  }
}
