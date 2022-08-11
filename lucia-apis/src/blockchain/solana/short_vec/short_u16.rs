use crate::blockchain::solana::short_vec::ShortU16Visitor;
use serde::{de::Deserializer, ser::SerializeTuple, Deserialize, Serialize, Serializer};

/// Same as u16, but serialized with 1 to 3 bytes. If the value is above
/// 0x7f, the top bit is set and the remaining value is stored in the next
/// bytes. Each byte follows the same pattern until the 3rd byte. The 3rd
/// byte, if needed, uses all 8 bits to store the last byte of the original
/// value.
pub(crate) struct ShortU16(pub(crate) u16);

impl<'de> Deserialize<'de> for ShortU16 {
  fn deserialize<D>(deserializer: D) -> Result<ShortU16, D::Error>
  where
    D: Deserializer<'de>,
  {
    deserializer.deserialize_tuple(3, ShortU16Visitor)
  }
}

impl Serialize for ShortU16 {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut seq = serializer.serialize_tuple(1)?;

    let mut rem_val = self.0;
    loop {
      let mut elem = (rem_val & 0x7f).to_ne_bytes()[0];
      rem_val = rem_val.wrapping_shr(7);
      if rem_val == 0 {
        seq.serialize_element(&elem)?;
        break;
      } else {
        elem |= 0x80;
        seq.serialize_element(&elem)?;
      }
    }
    seq.end()
  }
}
