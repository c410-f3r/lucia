use crate::utils::ByteBuffer;

/// Marker trait that has different bounds according to the given set of enabled serializers.
pub trait Serialize<DRSR> {
  /// Tries to encode itself into the specified amount of mutable bytes.
  fn to_bytes<B>(&self, bytes: &mut B, drsr: &mut DRSR) -> crate::Result<()>
  where
    B: ByteBuffer;
}

impl<DRSR, T> Serialize<DRSR> for &'_ T
where
  T: Serialize<DRSR>,
{
  #[inline]
  fn to_bytes<B>(&self, bytes: &mut B, drsr: &mut DRSR) -> crate::Result<()>
  where
    B: ByteBuffer,
  {
    (*self).to_bytes(bytes, drsr)
  }
}

impl<DRSR> Serialize<DRSR> for () {
  #[inline]
  fn to_bytes<B>(&self, _: &mut B, _: &mut DRSR) -> crate::Result<()>
  where
    B: ByteBuffer,
  {
    Ok(())
  }
}
