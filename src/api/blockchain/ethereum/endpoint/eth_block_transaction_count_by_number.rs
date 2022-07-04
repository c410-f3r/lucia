use crate::api::blockchain::ethereum::{BlockNumber, Ethereum};
use ethereum_types::U256;

_create_json_rpc_endpoint! {
  Ethereum;

  #[serde(transparent)]
  "eth_getBlockTransactionCountByNumber" => EthBlockTransactionCountByNumberReq<;;>([BlockNumber; 1])

  |raw: Option<U256>| -> Option<U256> { raw }

  eth_block_transaction_count_by_number(block_number: BlockNumber) {
    EthBlockTransactionCountByNumberReq([block_number])
  }
}
