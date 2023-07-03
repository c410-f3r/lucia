#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getStakeActivation")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{Commitment, SolanaHttpPkgsAux, StakeActivationState};

  #[pkg::aux]
  impl<DRSR> SolanaHttpPkgsAux<DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetStakeActivationReq<S>(
    #[pkg::field(name = "address")] S,
    #[pkg::field(name = "conf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    Option<GetStakeActivationConfig>,
  );

  #[derive(Debug, serde::Deserialize)]
  #[pkg::res_data]
  #[serde(rename_all = "camelCase")]
  pub struct GetStakeActivationRes {
    /// State
    pub state: StakeActivationState,
    /// Stake active during the epoch
    pub active: u64,
    /// Stake inactive during the epoch
    pub inactive: u64,
  }

  #[derive(Debug, serde::Serialize)]
  #[doc = generic_config_doc!()]
  #[serde(rename_all = "camelCase")]
  pub struct GetStakeActivationConfig {
    #[doc = commitment_doc!()]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    /// Epoch for which to calculate activation details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epoch: Option<u64>,
    #[doc = min_context_slot_doc!()]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_context_slot: Option<u64>,
  }
}
