#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getClusterNodes")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{SolanaAddressHashStr, SolanaHttpPkgsAux};
  use arrayvec::ArrayString;

  #[pkg::aux]
  impl<DRSR> SolanaHttpPkgsAux<DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetClusterNodesReq;

  #[pkg::res_data]
  pub type GetClusterNodesRes = Vec<GetClusterNodes>;

  #[derive(Debug, serde::Deserialize, PartialEq)]
  #[doc = _generic_res_data_elem_doc!()]
  pub struct GetClusterNodes {
    /// Gossip network address.
    pub gossip: ArrayString<21>,
    /// Node Base58 public key.
    pub pubkey: SolanaAddressHashStr,
    /// JSON RPC network address of the node.
    pub rpc: Option<ArrayString<21>>,
    /// TPU network address.
    pub tpu: Option<ArrayString<21>>,
    /// The software version of the node.
    pub version: Option<ArrayString<16>>,
  }
}
