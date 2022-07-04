use crate::utils::{RequestCounter, RequestLimit};

/// A wrapper around [RequestsCounter] and [RequestLimit].
#[derive(Debug)]
pub struct RequestThrottling {
  /// See [RequestsCounter]
  pub rc: RequestCounter,
  /// See [RequestLimit]
  pub rl: RequestLimit,
}
