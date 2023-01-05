#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getEpochInfo")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{Commitment, SolanaHttpPkgsAux};

  #[pkg::aux]
  impl<DRSR> SolanaHttpPkgsAux<DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetEpochInfoReq(
    #[pkg::field(name = "commitment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    Option<Commitment>,
    #[pkg::field(name = "min_context_slot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    Option<u64>,
  );

  #[derive(Debug, PartialEq, serde::Deserialize)]
  #[pkg::res_data]
  #[serde(rename_all = "camelCase")]
  pub struct GetEpochInfoRes {
    /// Current slot.
    pub absolute_slot: u64,
    /// Current block height.
    pub block_height: u64,
    /// Current epoch.
    pub epoch: u64,
    /// Current slot relative to the start of the current epoch.
    pub slot_index: u64,
    /// The number of slots in this epoch.
    pub slots_in_epoch: u64,
    /// Total number of transactions processed without error since genesis.
    pub transaction_count: Option<u64>,
  }
}
