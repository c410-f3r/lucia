use crate::misc::ByteBuffer;

/// Marker trait that has different bounds according to the given set of enabled serializers.
pub trait Serialize<DRSR> {
  /// Tries to encode itself into the specified amount of mutable bytes.
  fn to_bytes<BB>(&mut self, bytes: &mut BB, drsr: &mut DRSR) -> crate::Result<()>
  where
    BB: ByteBuffer;
}

impl<DRSR> Serialize<DRSR> for () {
  #[inline]
  fn to_bytes<BB>(&mut self, _: &mut BB, _: &mut DRSR) -> crate::Result<()>
  where
    BB: ByteBuffer,
  {
    Ok(())
  }
}
