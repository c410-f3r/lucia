//! Utility functions and structures

pub(crate) mod concat_array_str;
pub(crate) mod slice_by_commas;

use core::any::type_name;
use lucia::create_packages_aux_wrapper;

pub(crate) const _MAX_ASSET_ABBR_LEN: usize = 10;
/// The maximum length of any string number representation.
pub(crate) const _MAX_NUMBER_LEN: usize = 31;

pub(crate) type _MaxAssetAbbr = arrayvec::ArrayString<_MAX_ASSET_ABBR_LEN>;
pub(crate) type _MaxAssetName = arrayvec::ArrayString<16>;
pub(crate) type _MaxAssetFullName = arrayvec::ArrayString<48>;
pub(crate) type _MaxNumberStr = arrayvec::ArrayString<_MAX_NUMBER_LEN>;
pub(crate) type _MaxPairAbbr = arrayvec::ArrayString<{ 2 * _MAX_ASSET_ABBR_LEN + 1 }>;
pub(crate) type _MaxUrl = arrayvec::ArrayString<96>;

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

/// Useful when a request returns an optional field but the actual usage is within a
/// [core::result::Result] context.
#[inline]
#[track_caller]
pub fn into_rslt<T>(opt: Option<T>) -> crate::Result<T> {
  opt.ok_or(crate::Error::NoInnerValue(type_name::<T>()))
}

#[cfg(feature = "bs58")]
#[inline]
pub(crate) fn deserialize_array_from_base58<'de, D, const N: usize>(
  deserializer: D,
) -> Result<[u8; N], D::Error>
where
  D: serde::Deserializer<'de>,
{
  use serde::de::Error;
  let s: &str = serde::Deserialize::deserialize(deserializer)?;
  let mut array = [0; N];
  bs58::decode(s)
    .into(&mut array)
    .ok()
    .and_then(|len| {
      if len != N {
        return None;
      }
      Some(())
    })
    .ok_or_else(|| D::Error::custom("Could not deserialize base58 hash string"))?;
  Ok(array)
}

pub(crate) fn _deserialize_from_str<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
  T: core::str::FromStr,
  T::Err: core::fmt::Display,
  D: serde::Deserializer<'de>,
{
  let s: &str = serde::Deserialize::deserialize(deserializer)?;
  T::from_str(s).map_err(serde::de::Error::custom)
}

#[inline]
pub(crate) fn _deserialize_ignore_any<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
  D: serde::Deserializer<'de>,
  T: Default,
{
  use serde::Deserialize;
  serde::de::IgnoredAny::deserialize(deserializer).map(|_| T::default())
}

#[inline]
pub(crate) fn _deserialize_opt_considering_empty_str<'de, D, T>(
  deserializer: D,
) -> Result<Option<T>, D::Error>
where
  D: serde::Deserializer<'de>,
  T: serde::Deserialize<'de>,
{
  use serde::{de::IntoDeserializer, Deserialize};
  match <Option<&str>>::deserialize(deserializer)? {
    None | Some("") => Ok(None),
    Some(s) => T::deserialize(s.into_deserializer()).map(Some),
  }
}

#[cfg(test)]
#[inline]
pub(crate) fn _init_tracing() {
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

#[inline]
pub(crate) fn _serialize_as_tuple<T, S>(field: T, serializer: S) -> Result<S::Ok, S::Error>
where
  T: serde::Serialize,
  S: serde::Serializer,
{
  use serde::Serialize;
  (field,).serialize(serializer)
}

create_packages_aux_wrapper!();
