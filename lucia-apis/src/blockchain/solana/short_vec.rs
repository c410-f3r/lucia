mod short_u16;
mod short_u16_visitor;
mod short_vec_visitor;

use cl_aux::{Push, SingleTypeStorage, WithCapacity};
use core::marker::PhantomData;
use serde::{
  de::{self, Deserializer, SeqAccess},
  ser::{self, SerializeTuple, Serializer},
  Deserialize, Serialize,
};
use short_u16::*;
use short_u16_visitor::*;
use short_vec_visitor::*;

const MAX_ENCODING_LENGTH: usize = 3;

#[inline]
pub(crate) fn serialize<S, T>(elements: &[T], serializer: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
  T: Serialize,
{
  let mut seq = serializer.serialize_tuple(1)?;

  let len_u16 = if let Ok(elem) = u16::try_from(elements.len()) {
    elem
  } else {
    return Err(ser::Error::custom("length larger than u16"));
  };

  let short_len = ShortU16(len_u16);
  seq.serialize_element(&short_len)?;

  for element in elements {
    seq.serialize_element(element)?;
  }
  seq.end()
}

#[inline]
pub(crate) fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
  D: Deserializer<'de>,
  T: Push<T::Item, Output = ()> + SingleTypeStorage + WithCapacity<Input = usize>,
  T::Item: Deserialize<'de>,
{
  let visitor = ShortVecVisitor(PhantomData);
  deserializer.deserialize_tuple(usize::MAX, visitor)
}

enum VisitError {
  Alias,
  ByteThreeContinues,
  Overflow(u32),
  TooLong(usize),
  TooShort(usize),
}

impl VisitError {
  fn into_de_error<'de, A>(self) -> A::Error
  where
    A: SeqAccess<'de>,
  {
    match self {
      VisitError::TooLong(len) => de::Error::invalid_length(len, &"three or fewer bytes"),
      VisitError::TooShort(len) => de::Error::invalid_length(len, &"more bytes"),
      VisitError::Overflow(val) => de::Error::invalid_value(
        de::Unexpected::Unsigned(val.into()),
        &"a value in the range [0, 65535]",
      ),
      VisitError::Alias => {
        de::Error::invalid_value(de::Unexpected::Other("alias encoding"), &"strict form encoding")
      }
      VisitError::ByteThreeContinues => de::Error::invalid_value(
        de::Unexpected::Other("continue signal on byte-three"),
        &"a terminal signal on or before byte-three",
      ),
    }
  }
}

enum VisitStatus {
  Done(u16),
  More(u16),
}

struct ShortVec<T>(T);

impl<'de, T> Deserialize<'de> for ShortVec<T>
where
  T: Push<T::Item, Output = ()> + SingleTypeStorage + WithCapacity<Input = usize>,
  T::Item: Deserialize<'de>,
{
  fn deserialize<D>(deserializer: D) -> Result<ShortVec<T>, D::Error>
  where
    D: Deserializer<'de>,
  {
    deserialize(deserializer).map(ShortVec)
  }
}

impl<T> Serialize for ShortVec<T>
where
  T: Serialize,
  T: AsRef<[T::Item]> + SingleTypeStorage,
  T::Item: Serialize,
{
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serialize(self.0.as_ref(), serializer)
  }
}

fn visit_byte(elem: u8, val: u16, nth_byte: usize) -> Result<VisitStatus, VisitError> {
  if elem == 0 && nth_byte != 0 {
    return Err(VisitError::Alias);
  }

  let val_u32: u32 = val.into();
  let elem_u32: u32 = elem.into();
  let elem_val = elem_u32 & 0x7f;
  let elem_done = (elem_u32 & 0x80) == 0;

  if nth_byte >= MAX_ENCODING_LENGTH {
    return Err(VisitError::TooLong(nth_byte.saturating_add(1)));
  } else if nth_byte == MAX_ENCODING_LENGTH.saturating_sub(1) && !elem_done {
    return Err(VisitError::ByteThreeContinues);
  } else {
  }

  let shift = u32::try_from(nth_byte).unwrap_or(u32::MAX).saturating_mul(7);
  let shifted_elem_val = elem_val.checked_shl(shift).unwrap_or(u32::MAX);

  let new_val = val_u32 | shifted_elem_val;
  let final_val = u16::try_from(new_val).map_err(|_err| VisitError::Overflow(new_val))?;

  if elem_done {
    Ok(VisitStatus::Done(final_val))
  } else {
    Ok(VisitStatus::More(final_val))
  }
}
