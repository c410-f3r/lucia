#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getSlotLeaders")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{SolanaAddressHashStr, SolanaHttpPkgsAux};

  #[pkg::aux]
  impl<DRSR> SolanaHttpPkgsAux<DRSR> {}

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[pkg::req_data]
  pub struct GetSlotLeadersReq(#[pkg::field(name = "start")] u64, #[pkg::field(name = "len")] u64);

  #[pkg::res_data]
  pub type GetSlotLeadersRes = Vec<SolanaAddressHashStr>;
}
