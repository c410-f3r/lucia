use alloc::vec::Vec;

/// Value or Array
#[derive(Debug)]
pub struct ValueOrArray<T>(pub(crate) Vec<T>);

#[cfg(feature = "serde")]
mod serde {
  use crate::blockchain::ethereum::ValueOrArray;
  use serde::{Serialize, Serializer};

  impl<T> Serialize for ValueOrArray<T>
  where
    T: Serialize,
  {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer,
    {
      match self.0[..] {
        [] => serializer.serialize_none(),
        [ref elem] => Serialize::serialize(elem, serializer),
        _ => Serialize::serialize(&self.0, serializer),
      }
    }
  }
}
