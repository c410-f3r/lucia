#[allow(
  // Determined by the Solana devs
  variant_size_differences
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub enum Filter<'bytes> {
  DataSize(usize),
  #[cfg_attr(feature = "serde", serde(borrow))]
  Memcmp(Memcmp<'bytes>),
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug)]
pub struct Memcmp<'bytes> {
  #[cfg_attr(feature = "serde", serde(borrow))]
  pub bytes: MemcmpEncodedBytes<'bytes>,
  pub offset: usize,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase", untagged))]
#[derive(Debug)]
pub enum MemcmpEncodedBytes<'bytes> {
  Base58(&'bytes str),
  Base64(&'bytes str),
  Bytes(&'bytes [u8]),
}
