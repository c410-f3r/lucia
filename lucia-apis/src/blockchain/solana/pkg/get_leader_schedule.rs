#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getLeaderSchedule")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use std::collections::HashMap;

  use crate::blockchain::solana::{Commitment, SolanaBlockhash, SolanaHttpPkgsAux};
  use lucia::misc::AsyncTrait;

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetLeaderScheduleReq<S>(
    #[pkg::field(name = "slot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    Option<u64>,
    #[pkg::field(name = "conf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    Option<GetLeaderScheduleConfig<S>>,
  )
  where
    S: AsRef<str> + AsyncTrait;

  #[pkg::aux]
  impl<DRSR> SolanaHttpPkgsAux<DRSR> {}

  #[pkg::res_data]
  pub type GetLeaderScheduleRes = Option<HashMap<String, Vec<usize>>>;

  #[derive(Debug, serde::Deserialize)]
  #[serde(rename_all = "camelCase")]
  #[doc = _generic_res_data_elem_doc!()]
  pub struct GetLeaderSchedule {
    /// Block Base58 identifier.
    #[serde(deserialize_with = "crate::misc::deserialize_array_from_base58")]
    pub blockhash: SolanaBlockhash,
    /// Last block height at which the blockhash will be valid
    pub last_valid_block_height: u64,
  }

  #[derive(Debug, serde::Serialize)]
  #[doc = generic_config_doc!()]
  #[serde(rename_all = "camelCase")]
  pub struct GetLeaderScheduleConfig<S> {
    #[doc = commitment_doc!()]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    /// Only return results for this validator identity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<S>,
  }
}
