use crate::{consts::MAX_JSON_RPC_METHOD_LEN, protocol::JsonRpcNotificationParams};
use arrayvec::ArrayString;

/// A request object without an "id" member. Generally used with WebSocket connections.
#[derive(Debug, serde::Deserialize)]
pub struct JsonRpcNotification<R> {
  /// Name of the method invoked.
  pub method: Option<ArrayString<MAX_JSON_RPC_METHOD_LEN>>,
  /// See [crate::protocol::JsonRpcNotificationParams].
  pub params: JsonRpcNotificationParams<R>,
}
