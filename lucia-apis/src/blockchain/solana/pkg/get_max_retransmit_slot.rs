#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getMaxRetransmitSlot")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::SolanaHttpPkgsAux;

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetMaxRetransmitSlotReq;

  #[pkg::aux]
  impl<DRSR> SolanaHttpPkgsAux<DRSR> {}

  #[pkg::res_data]
  pub type GetMaxRetransmitSlotRes = u64;
}
