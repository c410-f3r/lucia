mod one_mand_and_one_opt;
mod request_manager_wrapper;

pub use one_mand_and_one_opt::OneMandAndOneOpt;
pub use request_manager_wrapper::RequestManagerWrapper;

use core::any::type_name;

pub(crate) const _MAX_ASSET_ABBR_LEN: usize = 10;
/// The maximum length of any string number representation.
pub(crate) const _MAX_NUMBER_LEN: usize = 31;

pub(crate) type _MaxAssetAbbr = arrayvec::ArrayString<_MAX_ASSET_ABBR_LEN>;
pub(crate) type _MaxAssetName = arrayvec::ArrayString<16>;
pub(crate) type _MaxNumberStr = arrayvec::ArrayString<_MAX_NUMBER_LEN>;
pub(crate) type _MaxPairAbbr = arrayvec::ArrayString<{ 2 * _MAX_ASSET_ABBR_LEN + 1 }>;
pub(crate) type _MaxUrl = arrayvec::ArrayString<96>;

_create_blockchain_constants!(
  pub address_hash: MaxAddressHash = 32,
  pub address_hash_str: MaxAddressHashStr = 46,
  pub block_hash: MaxBlockHash = 32,
  pub block_hash_str: MaxBlockHashStr = 67,
  pub signature_hash: MaxSignatureHash = 64,
  pub signature_hash_str: MaxSignatureHashStr = 90,
  pub transaction_hash: MaxTransactionHash = 64,
  pub transaction_hash_str: MaxTransactionHashStr = 90
);

/// Useful when a request returns an optional field that needs to be unwrapped in a
/// [core::result::Result] context.
#[inline]
#[track_caller]
pub fn into_rslt<T>(opt: Option<T>) -> crate::Result<T> {
  opt.ok_or(crate::Error::NoInnerValue(type_name::<T>()))
}

#[cfg(all(feature = "bs58", feature = "serde"))]
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

#[cfg(feature = "serde")]
#[inline]
pub(crate) fn _deserialize_ignore_any<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
  D: serde::Deserializer<'de>,
  T: Default,
{
  use serde::Deserialize;
  serde::de::IgnoredAny::deserialize(deserializer).map(|_| T::default())
}

#[cfg(test)]
#[inline]
pub(crate) fn _init_tracing() {
  use tracing_subscriber::{
    fmt::{format::FmtSpan, Subscriber},
    util::SubscriberInitExt,
    EnvFilter,
  };
  let _ = Subscriber::builder()
    .with_env_filter(EnvFilter::from_default_env())
    .with_span_events(FmtSpan::CLOSE | FmtSpan::NEW)
    .finish()
    .try_init();
}
