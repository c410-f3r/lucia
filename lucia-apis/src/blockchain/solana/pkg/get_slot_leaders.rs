#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getSlotLeaders")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{SolanaAddressHashStr, SolanaHttpPackagesAux};

  #[pkg::aux]
  impl<DRSR> SolanaHttpPackagesAux<DRSR> {}

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  #[pkg::req_data]
  pub struct GetSlotLeadersReqData(
    #[pkg::field(name = "start")] u64,
    #[pkg::field(name = "len")] u64,
  );

  #[lucia_macros::pkg_doc]
  #[pkg::res_data]
  pub type GetSlotLeadersResData = Vec<SolanaAddressHashStr>;
}
