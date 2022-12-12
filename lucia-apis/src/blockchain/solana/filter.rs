/// Filters results using additive objects.
#[allow(
  // Determined by the Solana devs
  variant_size_differences
)]
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Filter<'bytes> {
  /// Compares the program account data length with the provided data size.
  DataSize(usize),
  /// Memory comparison
  #[serde(borrow)]
  Memcmp(Memcmp<'bytes>),
}

/// Compares a provided series of bytes with program account data at a particular offset.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Memcmp<'bytes> {
  #[serde(borrow)]
  /// Encoded bytes
  pub bytes: MemcmpEncodedBytes<'bytes>,
  /// Offset into program account data to start comparison
  pub offset: usize,
}

/// Encoded bytes classified by its type.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum MemcmpEncodedBytes<'bytes> {
  /// Base58
  Base58(&'bytes str),
  /// Base64
  Base64(&'bytes str),
  /// Raw bytes
  Bytes(&'bytes [u8]),
}
