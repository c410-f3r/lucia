//! Utility functions and structures

mod concat_array_str;
mod slice_by_commas;

use arrayvec::ArrayString;
pub use concat_array_str::*;
use core::{fmt::Display, str::FromStr};
use lucia::create_packages_aux_wrapper;
use serde::{de::IntoDeserializer, Deserialize, Deserializer, Serialize, Serializer};
pub use slice_by_commas::*;

const MAX_ASSET_ABBR_LEN: usize = 10;
const MAX_NUMBER_LEN: usize = 31;

/// Maximum asset abbreviation like BTC.
pub type MaxAssetAbbr = ArrayString<MAX_ASSET_ABBR_LEN>;
/// Maximum asset name like Bitcoin.
pub type MaxAssetName = ArrayString<36>;
/// Maximum string representation of a number.
pub type MaxNumberStr = ArrayString<MAX_NUMBER_LEN>;
/// Maximum pair abbreviation like ETH-BTC
pub type MaxPairAbbr = ArrayString<{ 2 * MAX_ASSET_ABBR_LEN + 1 }>;

_create_blockchain_constants!(
  pub address_hash: MaxAddressHash = 32,
  pub address_hash_str: MaxAddressHashStr = 66,
  pub block_hash: MaxBlockHash = 32,
  pub block_hash_str: MaxBlockHashStr = 67,
  pub signature_hash: MaxSignatureHash = 64,
  pub signature_hash_str: MaxSignatureHashStr = 90,
  pub transaction_hash: MaxTransactionHash = 64,
  pub transaction_hash_str: MaxTransactionHashStr = 90
);

create_packages_aux_wrapper!();

/// Deserializes an Base58 string as an array of bytes.
#[cfg(feature = "bs58")]
#[inline]
pub fn deserialize_array_from_base58<'de, D, const N: usize>(
  deserializer: D,
) -> Result<[u8; N], D::Error>
where
  D: Deserializer<'de>,
{
  let s: &str = Deserialize::deserialize(deserializer)?;
  let mut array = [0; N];
  bs58::decode(s)
    .onto(&mut array)
    .ok()
    .and_then(|len| {
      if len != N {
        return None;
      }
      Some(())
    })
    .ok_or_else(|| serde::de::Error::custom("Could not deserialize base58 hash string"))?;
  Ok(array)
}

/// Deserializes an arbitrary type from a string.
#[inline]
pub fn deserialize_from_str<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
  T: FromStr,
  T::Err: Display,
  D: Deserializer<'de>,
{
  let s: &str = Deserialize::deserialize(deserializer)?;
  T::from_str(s).map_err(serde::de::Error::custom)
}

/// Deserializes an arbitrary type ignoring its contents.
#[inline]
pub fn deserialize_ignore_any<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
  D: Deserializer<'de>,
  T: Default,
{
  serde::de::IgnoredAny::deserialize(deserializer).map(|_| T::default())
}

/// Deserializes an arbitrary type from an optional string.
///
/// If the deserialized string is empty, then returns `None`.
#[inline]
pub fn deserialize_opt_considering_empty_str<'de, D, T>(
  deserializer: D,
) -> Result<Option<T>, D::Error>
where
  D: Deserializer<'de>,
  T: Deserialize<'de>,
{
  match <Option<&str>>::deserialize(deserializer)? {
    None | Some("") => Ok(None),
    Some(s) => T::deserialize(s.into_deserializer()).map(Some),
  }
}

/// Serializes an arbitrary type as a tuple
#[inline]
pub fn serialize_as_tuple<T, S>(field: T, serializer: S) -> Result<S::Ok, S::Error>
where
  T: Serialize,
  S: Serializer,
{
  (field,).serialize(serializer)
}

#[allow(dead_code)]
#[cfg(test)]
pub(crate) fn init_test_cfg() {
  use tracing_subscriber::{
    fmt::{format::FmtSpan, Subscriber},
    util::SubscriberInitExt,
    EnvFilter,
  };
  let _rslt = Subscriber::builder()
    .with_env_filter(EnvFilter::from_default_env())
    .with_span_events(FmtSpan::CLOSE | FmtSpan::NEW)
    .finish()
    .try_init();
}
