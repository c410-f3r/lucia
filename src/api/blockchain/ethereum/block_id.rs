use crate::api::blockchain::ethereum::BlockNumber;
use alloc::format;
use ethereum_types::H256;
use serde::{ser::SerializeStruct, Serialize, Serializer};

/// Block Identifier
#[derive(Clone, Copy, Debug)]
pub enum BlockId {
  /// By Hash
  Hash(H256),
  /// By Number
  Number(BlockNumber),
}

impl Serialize for BlockId {
  #[inline]
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    match *self {
      BlockId::Hash(ref x) => {
        let mut s = serializer.serialize_struct("BlockIdEip1898", 1)?;
        s.serialize_field("blockHash", &format!("{:?}", x))?;
        s.end()
      }
      BlockId::Number(ref num) => num.serialize(serializer),
    }
  }
}

impl From<u64> for BlockId {
  #[inline]
  fn from(num: u64) -> Self {
    BlockNumber::Number(num).into()
  }
}

impl From<BlockNumber> for BlockId {
  #[inline]
  fn from(num: BlockNumber) -> Self {
    BlockId::Number(num)
  }
}

impl From<H256> for BlockId {
  #[inline]
  fn from(hash: H256) -> Self {
    BlockId::Hash(hash)
  }
}
