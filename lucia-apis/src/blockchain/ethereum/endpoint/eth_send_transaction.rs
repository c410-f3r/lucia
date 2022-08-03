use crate::blockchain::ethereum::{Ethereum, TransactionRequest};
use ethereum_types::H256;

_create_json_rpc_endpoint! {
  Ethereum;

  #[cfg_attr(feature = "serde", serde(transparent))]
  "eth_sendTransaction" => EthSendTransactionReq<'tr;;>([&'tr TransactionRequest; 1])

  |raw: Option<H256>| -> Option<H256> { raw }

  eth_send_transaction(filter: &'tr TransactionRequest) {
    EthSendTransactionReq([filter])
  }
}
