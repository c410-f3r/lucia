use core::fmt::{Display, Formatter};

#[derive(Debug)]
/// Utility that displays slices separated by commas.
pub(crate) struct SliceByCommas<'any, T>(pub(crate) &'any [T]);

impl<T> Display for SliceByCommas<'_, T>
where
  T: Display,
{
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    let iter = &mut self.0.iter();
    if let Some(elem) = iter.next() {
      f.write_fmt(format_args!("{elem}"))?;
    }
    for elem in iter {
      f.write_fmt(format_args!(",{elem}"))?;
    }
    Ok(())
  }
}
