use core::fmt::{Display, Formatter};
use serde::Serialize;

#[derive(Debug)]
/// Utility that serializes strings as a single string.
pub struct ConcatArrayStr<'any, const N: usize>(
  /// Array
  pub [&'any str; N],
);

impl<const N: usize> Display for ConcatArrayStr<'_, N> {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    for elem in self.0 {
      f.write_str(elem)?;
    }
    Ok(())
  }
}

impl<const N: usize> Serialize for ConcatArrayStr<'_, N> {
  #[inline]
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.collect_str(self)
  }
}
