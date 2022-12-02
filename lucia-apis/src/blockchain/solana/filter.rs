/// Filters results using additive objects.
#[allow(
  // Determined by the Solana devs
  variant_size_differences
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub enum Filter<'bytes> {
  /// Compares the program account data length with the provided data size.
  DataSize(usize),
  /// Memory comparison
  #[cfg_attr(feature = "serde", serde(borrow))]
  Memcmp(Memcmp<'bytes>),
}

/// Compares a provided series of bytes with program account data at a particular offset.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug)]
pub struct Memcmp<'bytes> {
  #[cfg_attr(feature = "serde", serde(borrow))]
  /// Encoded bytes
  pub bytes: MemcmpEncodedBytes<'bytes>,
  /// Offset into program account data to start comparison
  pub offset: usize,
}

/// Encoded bytes classified by its type.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase", untagged))]
#[derive(Debug)]
pub enum MemcmpEncodedBytes<'bytes> {
  /// Base58
  Base58(&'bytes str),
  /// Base64
  Base64(&'bytes str),
  /// Raw bytes
  Bytes(&'bytes [u8]),
}
