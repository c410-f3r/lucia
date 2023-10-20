#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getInflationReward")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{Commitment, SolanaHttpPkgsAux};
  use lucia::misc::AsyncBounds;

  #[pkg::aux]
  impl<DRSR> SolanaHttpPkgsAux<DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetInflationRewardReq<S>(
    #[pkg::field(name = "pks")] S,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[pkg::field(name = "config")]
    Option<GetInflationRewardConfig>,
  )
  where
    S: AsyncBounds;

  #[pkg::res_data]
  pub type GetInflationRewardRes = Vec<GetInflationReward>;

  #[derive(Debug, serde::Serialize)]
  #[doc = generic_config_doc!()]
  #[serde(rename_all = "camelCase")]
  pub struct GetInflationRewardConfig {
    #[doc = commitment_doc!()]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    /// An epoch for which the reward occurs. If omitted, the previous epoch will be used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epoch: Option<u64>,
    #[doc = min_context_slot_doc!()]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_context_slot: Option<u64>,
  }

  #[derive(Debug, PartialEq, serde::Deserialize)]
  #[doc = _generic_res_data_elem_doc!()]
  pub struct GetInflationReward {
    /// Epoch for which reward occured.
    pub epoch: u64,
    /// The slot in which the rewards are effective.
    pub effective_slot: u64,
    /// Reward amount in lamports.
    pub amount: u64,
    /// Post balance of the account in lamports.
    pub post_balance: u64,
    /// Vote account commission when the reward was credited.
    pub commission: Option<u8>,
  }
}
