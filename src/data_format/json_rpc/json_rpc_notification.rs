use crate::{consts::MAX_JSON_RPC_METHOD_LEN, data_format::JsonRpcNotificationParams};
use arrayvec::ArrayString;

/// A request object without an "id" member. Generally used with WebSocket connections.
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Debug)]
pub struct JsonRpcNotification<R> {
  /// Name of the method invoked.
  pub method: Option<ArrayString<MAX_JSON_RPC_METHOD_LEN>>,
  /// See [crate::data_format::JsonRpcNotificationParams].
  pub params: JsonRpcNotificationParams<R>,
}
