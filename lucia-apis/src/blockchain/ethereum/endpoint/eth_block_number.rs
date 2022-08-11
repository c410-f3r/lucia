use crate::blockchain::ethereum::Ethereum;
use ethereum_types::U64;

_create_json_rpc_endpoint! {
  Ethereum;

  "eth_blockNumber" => EthBlockNumberReq<;;>

  |raw: Option<U64>| -> Option<u64> { raw.map(|el| el.as_u64()) }

  eth_block_number() { EthBlockNumberReq }
}
