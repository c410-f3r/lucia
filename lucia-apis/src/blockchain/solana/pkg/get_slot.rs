#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getSlot")),
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
  pub struct GetSlotReq(
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[pkg::field(name = "commitment")]
    pub Option<Commitment>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[pkg::field(name = "min_context_slot")]
    pub Option<u64>,
  );

  #[pkg::res_data]
  pub type GetSlotRes = u64;
}
