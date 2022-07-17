use core::{any::type_name, fmt::Formatter, marker::PhantomData};
use serde::{
  de::{Error as _, SeqAccess, Visitor},
  Deserialize,
};

pub(crate) struct SeqVisitor<F, T>(F, PhantomData<T>);

impl<F, T> SeqVisitor<F, T> {
  #[inline]
  pub(crate) fn new(cb: F) -> Self {
    Self(cb, PhantomData)
  }
}

impl<'de, F, T> Visitor<'de> for SeqVisitor<F, T>
where
  F: FnMut(T) -> crate::Result<()>,
  T: Deserialize<'de>,
{
  type Value = ();

  #[inline]
  fn expecting(&self, formatter: &mut Formatter<'_>) -> core::fmt::Result {
    formatter.write_fmt(format_args!("generic sequence of {}", type_name::<T>()))
  }

  #[inline]
  fn visit_seq<A>(mut self, mut seq: A) -> Result<Self::Value, A::Error>
  where
    A: SeqAccess<'de>,
  {
    while let Some(elem) = seq.next_element::<T>()? {
      (self.0)(elem).map_err(A::Error::custom)?;
    }
    Ok(())
  }
}
