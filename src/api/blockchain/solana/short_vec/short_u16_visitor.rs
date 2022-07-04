use crate::api::blockchain::solana::short_vec::{
  visit_byte, ShortU16, VisitError, VisitStatus, MAX_ENCODING_LENGTH,
};
use core::fmt::Formatter;
use serde::de::{SeqAccess, Visitor};

pub(crate) struct ShortU16Visitor;

impl<'de> Visitor<'de> for ShortU16Visitor {
  type Value = ShortU16;

  fn expecting(&self, formatter: &mut Formatter<'_>) -> core::fmt::Result {
    formatter.write_str("a ShortU16")
  }

  fn visit_seq<A>(self, mut seq: A) -> Result<ShortU16, A::Error>
  where
    A: SeqAccess<'de>,
  {
    // Decodes an unsigned 16 bit integer one-to-one encoded as follows:
    // 1 byte  : 0xxxxxxx                   => 00000000 0xxxxxxx :      0 -    127
    // 2 bytes : 1xxxxxxx 0yyyyyyy          => 00yyyyyy yxxxxxxx :    128 - 16,383
    // 3 bytes : 1xxxxxxx 1yyyyyyy 000000zz => zzyyyyyy yxxxxxxx : 16,384 - 65,535
    let mut val: u16 = 0;
    for nth_byte in 0..MAX_ENCODING_LENGTH {
      let elem: u8 = seq
        .next_element()?
        .ok_or_else(|| VisitError::TooShort(nth_byte.saturating_add(1)).into_de_error::<A>())?;
      match visit_byte(elem, val, nth_byte).map_err(|e| e.into_de_error::<A>())? {
        VisitStatus::Done(new_val) => return Ok(ShortU16(new_val)),
        VisitStatus::More(new_val) => val = new_val,
      }
    }

    Err(VisitError::ByteThreeContinues.into_de_error::<A>())
  }
}
