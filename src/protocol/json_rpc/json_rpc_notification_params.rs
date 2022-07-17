/// A Structured value that holds the parameter values to be used during the invocation of the
/// method.
#[derive(Debug, serde::Deserialize)]
pub struct JsonRpcNotificationParams<R> {
  /// Payload
  pub result: R,
  /// Identifier returned by the counterpart
  pub subscription: u64,
}
