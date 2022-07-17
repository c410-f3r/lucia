use alloc::{borrow::ToOwned, string::String, vec::Vec};
use core::fmt::{Debug, Formatter};
use serde::{
  de::{Error, Unexpected, Visitor},
  Deserialize, Deserializer, Serialize, Serializer,
};

/// Raw bytes wrapper
#[derive(Debug, Default, Eq, PartialEq)]
pub struct Bytes(pub Vec<u8>);

impl<'de> Deserialize<'de> for Bytes {
  #[inline]
  fn deserialize<D>(deserializer: D) -> Result<Bytes, D::Error>
  where
    D: Deserializer<'de>,
  {
    deserializer.deserialize_identifier(BytesVisitor)
  }
}

impl<T> From<T> for Bytes
where
  T: Into<Vec<u8>>,
{
  #[inline]
  fn from(data: T) -> Self {
    Bytes(data.into())
  }
}

impl Serialize for Bytes {
  #[inline]
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut serialized = "0x".to_owned();
    serialized.push_str(&hex::encode(&self.0));
    serializer.serialize_str(serialized.as_ref())
  }
}

struct BytesVisitor;

impl<'de> Visitor<'de> for BytesVisitor {
  type Value = Bytes;

  fn expecting(&self, formatter: &mut Formatter<'_>) -> core::fmt::Result {
    write!(formatter, "a 0x-prefixed hex-encoded vector of bytes")
  }

  fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
  where
    E: Error,
  {
    if value.len() >= 2 && value.get(..2).unwrap_or_default() == "0x" {
      Ok(Bytes(hex::decode(value.get(2..).unwrap_or_default()).map_err(|err| E::custom(err))?))
    } else {
      Err(E::invalid_value(Unexpected::Str(value), &"0x prefix"))
    }
  }

  fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
  where
    E: Error,
  {
    self.visit_str(value.as_ref())
  }
}
