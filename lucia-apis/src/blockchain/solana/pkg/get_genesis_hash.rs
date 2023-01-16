#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getGenesisHash")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::{blockchain::solana::SolanaHttpPkgsAux, misc::MaxBlockHashStr};

  #[pkg::aux]
  impl<DRSR> SolanaHttpPkgsAux<DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetGenesisHashReq;

  #[pkg::res_data]
  pub type GetGenesisHashRes = MaxBlockHashStr;
}
