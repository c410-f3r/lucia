#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getSlotLeader")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{Commitment, SolanaAddressHashStr, SolanaHttpPkgsAux};

  #[pkg::aux]
  impl<DRSR> SolanaHttpPkgsAux<DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetSlotLeaderReq(
    #[pkg::field(name = "config")]
    #[serde(serialize_with = "crate::misc::serialize_as_tuple")]
    #[serde(skip_serializing_if = "Option::is_none")]
    Option<GetSlotLeaderConfig>,
  );

  #[pkg::res_data]
  pub type GetSlotLeaderRes = SolanaAddressHashStr;

  #[derive(Debug, serde::Serialize)]
  #[doc = generic_config_doc!()]
  #[serde(rename_all = "camelCase")]
  pub struct GetSlotLeaderConfig {
    #[doc = commitment_doc!()]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    #[doc = min_context_slot_doc!()]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_context_slot: Option<u64>,
  }
}
