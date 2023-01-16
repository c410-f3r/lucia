#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getInflationGovernor")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::{Commitment, SolanaHttpPkgsAux};

  #[pkg::aux]
  impl<DRSR> SolanaHttpPkgsAux<DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetInflationGovernorReq(
    #[pkg::field(name = "commitment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    Option<Commitment>,
  );

  #[derive(Debug, PartialEq, serde::Deserialize)]
  #[pkg::res_data]
  #[serde(rename_all = "camelCase")]
  pub struct GetInflationGovernorRes {
    /// Initial inflation percentage from time 0.
    pub initial: f64,
    /// Terminal inflation percentage.
    pub terminal: f64,
    /// Rate per year at which inflation is lowered. Rate reduction is derived using the target
    /// slot time in genesis config.
    pub taper: f64,
    /// Percentage of total inflation allocated to the foundation.
    pub foundation: f64,
    /// Duration of foundation pool inflation in years.
    pub foundation_term: f64,
  }
}
