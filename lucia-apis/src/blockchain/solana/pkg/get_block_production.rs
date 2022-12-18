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
    #[pkg::field(name = "config")]
    #[serde(serialize_with = "crate::misc::_serialize_as_tuple")]
    #[serde(skip_serializing_if = "Option::is_none")]
    Option<GetBlockProductionConfig<'any>>,
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

  #[derive(Debug, serde::Serialize)]
  #[doc = generic_config_doc!()]
  pub struct GetBlockProductionConfig<'any> {
    #[doc = commitment_doc!()]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    /// Only return results for this validator identity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<&'any str>,
    /// Slot range to return block production for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<GetBlockProductionRange>,
  }

  /// Block production slot range
  #[derive(Debug, serde::Deserialize, serde::Serialize)]
  #[serde(rename_all = "camelCase")]
  pub struct GetBlockProductionRange {
    /// First slot of the block production information (inclusive)
    pub first_slot: u64,
    /// Last slot of block production information (inclusive)
    pub last_slot: Option<u64>,
  }
}
