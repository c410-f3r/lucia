use crate::{
  blockchain::ethereum::{BlockNumber, Ethereum},
  misc::OneMandAndOneOpt,
};
use ethereum_types::U256;

_create_json_rpc_endpoint! {
  Ethereum;

  "eth_getBalance" => EthGetBalanceReqParams<;;S AsRef<str> = &'static str>(
    OneMandAndOneOpt<S, BlockNumber>
  )

  |raw: Wrapper| -> U256 { raw.0 }

  eth_get_balance(addr: S, bn: Option<BlockNumber>) {
    EthGetBalanceReqParams(OneMandAndOneOpt(addr, bn))
  }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Debug)]
pub struct Wrapper(U256);
