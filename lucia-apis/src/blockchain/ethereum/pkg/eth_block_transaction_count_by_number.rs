#[lucia_macros::pkg(
  api(crate::blockchain::ethereum::Ethereum),
  data_format(json_rpc("eth_getBlockTransactionCountByNumber")),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::blockchain::ethereum::{BlockNumber, EthereumHttpPkgsAux};
  use ethereum_types::U256;

  #[pkg::aux]
  impl<DRSR> EthereumHttpPkgsAux<DRSR> {}

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct EthBlockTransactionCountByNumberReq<'any>(
    #[pkg::field(name = "block_number")] [&'any BlockNumber; 1],
  );

  #[pkg::res_data]
  pub type EthBlockTransactionCountByNumberRes = Option<U256>;
}
