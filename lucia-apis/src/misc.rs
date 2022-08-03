pub(crate) mod consts;
pub(crate) mod types;

mod one_mand_and_one_opt;
mod request_manager_wrapper;

pub use one_mand_and_one_opt::OneMandAndOneOpt;
pub use request_manager_wrapper::RequestManagerWrapper;

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
pub(crate) fn deserialize_ignore_any<'de, D, T>(deserializer: D) -> Result<T, D::Error>
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
