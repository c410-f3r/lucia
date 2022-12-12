#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("GetBlockCommitment")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{JsonRpcResponseResultWithContext, SolanaHttpPkgsAux};

  #[pkg::aux]
  impl<DRSR> SolanaHttpPkgsAux<DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetBlockCommitmentReq(
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pkg::field(name = "block")]
    u64,
  );

  #[pkg::res_data]
  pub type GetBlockCommitmentRes = JsonRpcResponseResultWithContext<GetBlockCommitment>;

  /// Block commitment
  #[derive(Debug, serde::Deserialize)]
  #[serde(rename_all = "camelCase")]
  pub struct GetBlockCommitment {
    /// Amount of cluster stake in lamports that has voted on the block .
    pub commitment: Option<Vec<u64>>,
    /// Total active stake, in lamports, of the current epoch.
    pub total_stake: u64,
  }
}
