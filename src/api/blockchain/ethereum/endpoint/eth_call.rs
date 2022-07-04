use crate::api::blockchain::ethereum::{BlockId, CallRequest, Ethereum};

_create_json_rpc_endpoint! {
  Ethereum;

  #[serde(transparent)]
  "eth_call" => EthCallReq<'call_request;;>((&'call_request CallRequest, Option<BlockId>))

  |raw: Option<crate::api::blockchain::ethereum::Bytes>| -> Option<crate::api::blockchain::ethereum::Bytes> {
    raw
  }

  eth_call(block_id: Option<BlockId>, call_request: &'call_request CallRequest) {
    EthCallReq((call_request, block_id))
  }
}
