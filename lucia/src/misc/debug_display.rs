use core::fmt::{Debug, Display};

/// Synchronous dynamic DebugDisplay
pub type SyncDynDebugDisplay = dyn DebugDisplay + Sync;

/// This trait only exists because `trait_alias` is not stable and also because serde does not
/// provide `Serialize` implementations for dynamic traits.
pub trait DebugDisplay: Debug + Display {}

impl<T> DebugDisplay for T where T: Debug + Display {}

#[cfg(feature = "serde")]
impl serde::Serialize for &'_ dyn DebugDisplay {
  #[inline]
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.collect_str(&format_args!("{}", self))
  }
}

#[cfg(feature = "serde")]
impl serde::Serialize for &'_ SyncDynDebugDisplay {
  #[inline]
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.collect_str(&format_args!("{}", self))
  }
}
