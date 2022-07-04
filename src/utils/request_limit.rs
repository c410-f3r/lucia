use core::time::Duration;

/// Determines how many times a series of requests can be performed within a certain duration
#[derive(Debug)]
pub struct RequestLimit {
  ms: Duration,
  limit: u64,
}

impl RequestLimit {
  /// New instance based on millis-seconds.
  #[inline]
  pub const fn from_ms(limit: u64, ms: u64) -> Self {
    Self { ms: Duration::from_millis(ms), limit }
  }

  /// The interval range that can contain a maximum number of [Self::limit] requests
  #[inline]
  pub const fn duration(&self) -> &Duration {
    &self.ms
  }

  /// Upper bound or maximum possible number of requests
  #[inline]
  pub const fn limit(&self) -> u64 {
    self.limit
  }
}
