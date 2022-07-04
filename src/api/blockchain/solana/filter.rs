#[allow(
  // Determined by the Solana devs
  variant_size_differences
)]
#[derive(Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Filter<'bytes> {
  DataSize(usize),
  #[serde(borrow)]
  Memcmp(Memcmp<'bytes>),
}

#[derive(Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Memcmp<'bytes> {
  #[serde(borrow)]
  pub bytes: MemcmpEncodedBytes<'bytes>,
  pub offset: usize,
}

#[derive(Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum MemcmpEncodedBytes<'bytes> {
  Base58(&'bytes str),
  Base64(&'bytes str),
  Bytes(&'bytes [u8]),
}
