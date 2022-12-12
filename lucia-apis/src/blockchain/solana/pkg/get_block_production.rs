#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getBlockProduction")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{
    Commitment, JsonRpcResponseResultWithContext, SolanaAddressHashStr, SolanaHttpPkgsAux,
  };
  use alloc::collections::BTreeMap;

  #[pkg::aux]
  impl<DRSR> SolanaHttpPkgsAux<DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetBlockProductionReq<'any>(
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pkg::field(name = "commitment")]
    Option<Commitment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pkg::field(name = "range")]
    Option<(u64, Option<u64>)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pkg::field(name = "identity")]
    Option<&'any str>,
  );

  #[pkg::res_data]
  pub type GetBlockProductionRes = JsonRpcResponseResultWithContext<Option<GetBlockProduction>>;

  /// Recent block production information from the current or previous epoch.
  #[derive(Debug, serde::Deserialize)]
  #[serde(rename_all = "camelCase")]
  pub struct GetBlockProduction {
    /// Map of leader base58 identity pubkeys to a tuple of `(number of leader slots, number of blocks produced)`
    pub by_identity: BTreeMap<SolanaAddressHashStr, [usize; 2]>,
    /// Range
    pub range: GetBlockProductionRange,
  }

  /// Block production slot range
  #[derive(Debug, serde::Deserialize)]
  #[serde(rename_all = "camelCase")]
  pub struct GetBlockProductionRange {
    /// First slot of the block production information (inclusive)
    pub first_slot: u64,
    /// Last slot of block production information (inclusive)
    pub last_slot: u64,
  }
}
