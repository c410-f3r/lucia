use crate::blockchain::ethereum::{Ethereum, Filter, Log};
use alloc::vec::Vec;

_create_json_rpc_endpoint! {
  Ethereum;

  #[cfg_attr(feature = "serde", serde(transparent))]
  "eth_getLogs" => EthGetLogsReq<'filter;;>(&'filter Filter)

  |raw: Option<Vec<Log>>| -> Option<Vec<Log>> { raw }

  eth_get_logs(filter: &'filter Filter) { EthGetLogsReq(filter) }
}
