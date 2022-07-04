#[derive(Debug, serde::Deserialize)]
pub struct JsonRpcNotificationParams<R> {
  pub result: R,
  pub subscription: u64,
}
