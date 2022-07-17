#[allow(
  // Probably little will be gained boxing a variant of 128 bits
  variant_size_differences
)]
/// Transactions usually take some time to be confirmed.
#[derive(Debug)]
pub enum ConfirmTransactionOptions {
  /// Keeps trying fetching a transaction until a provided `number` of iteration is reached or the
  /// transaction is confirmed.
  Tries {
    /// Max of 65_535 requests
    number: u16,
  },
  /// Keeps trying fetching a transaction until a provided `number` of iteration is reached or the
  /// transaction is confirmed. Each iteration awaits the provided `interval`.
  TriesWithInterval {
    /// Any measure of time
    interval: core::time::Duration,
    /// Max of 65_535 requests
    number: u16,
  },
}
