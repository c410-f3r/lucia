#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getBlockHeight")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{Commitment, SolanaHttpPkgsAux};

  #[pkg::aux]
  impl<DRSR> SolanaHttpPkgsAux<DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetBlockHeightReq(
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pkg::field(name = "commitment")]
    Option<Commitment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pkg::field(name = "min_context_slot")]
    Option<u64>,
  );

  #[pkg::res_data]
  pub type GetBlockHeightRes = u64;
}
