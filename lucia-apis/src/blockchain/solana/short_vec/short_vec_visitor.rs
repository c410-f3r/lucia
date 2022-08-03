use crate::blockchain::solana::short_vec::ShortU16;
use cl_aux::{Push, SingleTypeStorage, WithCapacity};
use core::{fmt, marker::PhantomData};
use serde::{
  de::{self, SeqAccess, Visitor},
  Deserialize,
};

pub(crate) struct ShortVecVisitor<T>(pub(crate) PhantomData<T>);

impl<'de, T> Visitor<'de> for ShortVecVisitor<T>
where
  T: Push<T::Item, Output = ()> + SingleTypeStorage + WithCapacity<Input = usize>,
  T::Item: Deserialize<'de>,
{
  type Value = T;

  fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str("a vector with a multi-byte length")
  }

  fn visit_seq<A>(self, mut seq: A) -> Result<T, A::Error>
  where
    A: SeqAccess<'de>,
  {
    let short_u16: ShortU16 =
      seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
    let len: usize = short_u16.0.into();

    let mut result = T::with_capacity(len);
    for i in 0..len {
      let elem = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(i, &self))?;
      result.push(elem).map_err(|_err| de::Error::custom("Insufficient space"))?;
    }
    Ok(result)
  }
}
