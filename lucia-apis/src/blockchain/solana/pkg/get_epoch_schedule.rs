#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getEpochSchedule")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::SolanaHttpPkgsAux;

  #[pkg::aux]
  impl<DRSR> SolanaHttpPkgsAux<DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetEpochScheduleReq;

  #[derive(Debug, PartialEq, serde::Deserialize)]
  #[pkg::res_data]
  #[serde(rename_all = "camelCase")]
  pub struct GetEpochScheduleRes {
    /// Maximum number of slots in each epoch.
    pub slots_per_epoch: u64,
    /// Number of slots before beginning of an epoch to calculate a leader schedule for that epoch.
    pub leader_schedule_slot_offset: u64,
    /// Whether epochs start short and grow.
    pub warmup: bool,
    /// First normal-length epoch, log2(slotsPerEpoch) - log2(MINIMUM_SLOTS_PER_EPOCH).
    pub first_normal_epoch: u64,
    /// MINIMUM_SLOTS_PER_EPOCH * (2.pow(firstNormalEpoch) - 1)
    pub first_normal_slot: u64,
  }
}
