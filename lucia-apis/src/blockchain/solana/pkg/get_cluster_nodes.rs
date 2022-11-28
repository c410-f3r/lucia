#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getClusterNodes")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{SolanaAddressHashStr, SolanaHttpPackagesAux};
  use arrayvec::ArrayString;

  #[pkg::aux]
  impl<DRSR> SolanaHttpPackagesAux<DRSR> {}

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  #[pkg::req_data]
  pub struct GetClusterNodesReqData;

  #[lucia_macros::pkg_doc]
  #[pkg::res_data]
  pub type GetClusterNodesResData = Vec<GetClusterNodesResElem>;

  #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
  #[derive(Debug, PartialEq)]
  #[lucia_macros::pkg_doc]
  pub struct GetClusterNodesResElem {
    pub gossip: ArrayString<21>,
    pub pubkey: SolanaAddressHashStr,
    pub rpc: Option<ArrayString<21>>,
    pub tpu: Option<ArrayString<21>>,
    pub version: Option<ArrayString<16>>,
  }
}
