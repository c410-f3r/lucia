#[lucia_macros::pkg(
  api(crate::blockchain::ethereum::Ethereum),
  data_format(json_rpc("eth_getBalance")),
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
  pub struct EthGetBalanceReq<'any, S>(
    #[pkg::field(name = "addr")] S,
    #[pkg::field(name = "block_number")] &'any BlockNumber,
  );

  #[pkg::res_data]
  pub type EthGetBalanceRes = Option<U256>;
}
