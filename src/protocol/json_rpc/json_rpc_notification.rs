use crate::{consts::MAX_JSON_RPC_METHOD_LEN, protocol::JsonRpcNotificationParams};
use arrayvec::ArrayString;

#[derive(Debug, serde::Deserialize)]
pub struct JsonRpcNotification<R> {
  pub method: Option<ArrayString<MAX_JSON_RPC_METHOD_LEN>>,
  pub params: JsonRpcNotificationParams<R>,
}
