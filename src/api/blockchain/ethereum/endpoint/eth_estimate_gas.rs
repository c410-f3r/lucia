use crate::{
  api::blockchain::ethereum::{BlockNumber, CallRequest, Ethereum},
  utils::OneMandAndOneOpt,
};
use ethereum_types::U256;

_create_json_rpc_endpoint! {
  Ethereum;

  #[cfg_attr(feature = "serde", serde(transparent))]
  "eth_estimateGas" => EthEstimateGasReq<'call_request;;>(OneMandAndOneOpt<&'call_request CallRequest, BlockNumber>)

  |raw: U256| -> U256 { raw }

  eth_estimate_gas(block_number: Option<BlockNumber>, call_request: &'call_request CallRequest) {
    EthEstimateGasReq(OneMandAndOneOpt(call_request, block_number))
  }
}
