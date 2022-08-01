/// Block Number
#[derive(Clone, Copy, Debug)]
pub enum BlockNumber {
  /// Earliest block (genesis)
  Earliest,
  /// Latest block
  Latest,
  /// Block by number from canon chain
  Number(u64),
  /// Pending block (not yet part of the blockchain)
  Pending,
}

impl<T> From<T> for BlockNumber
where
  T: Into<u64>,
{
  #[inline]
  fn from(num: T) -> Self {
    BlockNumber::Number(num.into())
  }
}

#[cfg(feature = "serde")]
mod serde {
  use crate::api::blockchain::ethereum::BlockNumber;
  use arrayvec::ArrayString;
  use core::fmt::Write;
  use serde::{ser::Error, Serialize, Serializer};

  impl Serialize for BlockNumber {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer,
    {
      match *self {
        BlockNumber::Number(ref x) => {
          let mut s = ArrayString::<10>::new();
          s.write_fmt(format_args!("0x{:x}", x))
            .map_err(|_err| S::Error::custom("Buffer is not large enough to fill block number"))?;
          serializer.serialize_str(s.as_str())
        }
        BlockNumber::Latest => serializer.serialize_str("latest"),
        BlockNumber::Earliest => serializer.serialize_str("earliest"),
        BlockNumber::Pending => serializer.serialize_str("pending"),
      }
    }
  }
}
