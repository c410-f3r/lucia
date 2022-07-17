use crate::{
  api::blockchain::ethereum::{BlockNumber, Ethereum},
  utils::OneMandAndOneOpt,
};
use ethereum_types::U256;

_create_json_rpc_endpoint! {
  Ethereum;

  "eth_getBalance" => EthGetBalanceReqParams<;;S AsRef<str> = &'static str>(
    OneMandAndOneOpt<S, BlockNumber>
  )

  |raw: U256| -> U256 { raw }

  eth_get_balance(addr: S, bn: Option<BlockNumber>) {
    EthGetBalanceReqParams(OneMandAndOneOpt(addr, bn))
  }
}
