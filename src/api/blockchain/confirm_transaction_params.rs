#[allow(
  // Probably little will be gained boxing a variant of 128 bits
  variant_size_differences
)]
#[derive(Debug, Eq, PartialEq)]
pub enum ConfirmTransactionOptions {
  /// Keeps trying fetching a transaction until a provided `number` of iteration is reached or the
  /// transaction is confirmed.
  Tries { number: u32 },
  /// Keeps trying fetching a transaction until a provided `number` of iteration is reached or the
  /// transaction is confirmed. Each iteration awaits the provided `interval`.
  #[cfg(any(feature = "tokio", feature = "async-std"))]
  TriesWithInterval { interval: core::time::Duration, number: u32 },
}
