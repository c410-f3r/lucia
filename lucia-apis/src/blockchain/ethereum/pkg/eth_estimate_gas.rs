#[lucia_macros::pkg(
  api(crate::blockchain::ethereum::Ethereum),
  data_format(json_rpc("eth_estimateGas")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::ethereum::{BlockNumber, CallRequest, EthereumHttpPkgsAux};
  use ethereum_types::U256;

  #[pkg::aux]
  impl<DRSR> EthereumHttpPkgsAux<DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct EthEstimateGasReq<'any>(
    #[pkg::field(name = "block_number")] Option<&'any BlockNumber>,
    #[pkg::field(name = "cr")] &'any CallRequest,
  );

  #[pkg::res_data]
  pub type EthEstimateGasRes = U256;
}
