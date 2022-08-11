use alloc::{string::String, vec::Vec};

/// This trait only exists because of the lack of `impl TryFrom<&[u8]> for String` but such
/// implementation probably will never be a thing.
///
/// Used by [crate::network::Test].
pub trait FromBytes {
  /// Creates itself from a sequence of bytes.
  fn from_bytes(bytes: &[u8]) -> crate::Result<Self>
  where
    Self: Sized;
}

impl FromBytes for String {
  #[inline]
  fn from_bytes(bytes: &[u8]) -> crate::Result<Self>
  where
    Self: Sized,
  {
    Ok(core::str::from_utf8(bytes)?.into())
  }
}

impl FromBytes for Vec<u8> {
  #[inline]
  fn from_bytes(bytes: &[u8]) -> crate::Result<Self>
  where
    Self: Sized,
  {
    Ok(bytes.into())
  }
}
