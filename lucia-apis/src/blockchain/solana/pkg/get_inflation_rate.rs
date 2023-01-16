#[lucia_macros::pkg(
  api(crate::blockchain::solana::Solana),
  data_format(json_rpc("getInflationRate")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::solana::SolanaHttpPkgsAux;

  #[pkg::aux]
  impl<DRSR> SolanaHttpPkgsAux<DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct GetInflationRateReq;

  #[derive(Debug, PartialEq, serde::Deserialize)]
  #[pkg::res_data]
  pub struct GetInflationRateRes {
    /// Total inflation
    pub total: f64,
    /// Inflation allocated to validators.
    pub validator: f64,
    /// Inflation allocated to the foundation
    pub foundation: f64,
    /// Epoch for which these values are valid.
    pub epoch: u64,
  }
}
