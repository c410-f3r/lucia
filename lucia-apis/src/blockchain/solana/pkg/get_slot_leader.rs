#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getSlotLeader")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{Commitment, SolanaAddressHashStr, SolanaHttpPkgsAux};

  #[pkg::aux]
  impl<DRSR> SolanaHttpPkgsAux<DRSR> {}

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[pkg::req_data]
  pub struct GetSlotLeaderReq(
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[pkg::field(name = "commitment")]
    pub Option<Commitment>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[pkg::field(name = "min_context_slot")]
    pub Option<u64>,
  );

  #[pkg::res_data]
  pub type GetSlotLeaderRes = SolanaAddressHashStr;
}
