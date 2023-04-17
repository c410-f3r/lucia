use crate::blockchain::solana::MessageInput;
use core::fmt::Formatter;
use serde::{
  de::{self, SeqAccess, Visitor},
  ser::SerializeTuple,
  Deserialize, Deserializer, Serialize, Serializer,
};

const MESSAGE_VERSION_PREFIX: u8 = 0x80;

/// Wrapper that tells witch version a message represents.
#[derive(Debug)]
pub enum VersionedMessageInput {
  /// V0
  V0(MessageInput),
}

impl<'de> Deserialize<'de> for VersionedMessageInput {
  fn deserialize<D>(deserializer: D) -> Result<VersionedMessageInput, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct MessageVisitor;

    impl<'de> Visitor<'de> for MessageVisitor {
      type Value = VersionedMessageInput;

      fn expecting(&self, formatter: &mut Formatter<'_>) -> core::fmt::Result {
        formatter.write_str("transaction message")
      }

      fn visit_seq<A>(self, mut seq: A) -> Result<VersionedMessageInput, A::Error>
      where
        A: SeqAccess<'de>,
      {
        let version = seq
          .next_element::<MessageVersion>()?
          .ok_or_else(|| de::Error::invalid_length(0, &self))?
          .0;
        match version {
          0 => Ok(VersionedMessageInput::V0(
            seq.next_element()?.ok_or_else(|| de::Error::invalid_length(1, &self))?,
          )),
          _ => Err(de::Error::custom(crate::Error::SolanaUnsupportedMessageFormat)),
        }
      }
    }
    deserializer.deserialize_tuple(2, MessageVisitor)
  }
}

impl Serialize for VersionedMessageInput {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    match self {
      Self::V0(msg) => {
        let mut seq = serializer.serialize_tuple(2)?;
        seq.serialize_element(&MESSAGE_VERSION_PREFIX)?;
        seq.serialize_element(msg)?;
        seq.end()
      }
    }
  }
}

impl From<MessageInput> for VersionedMessageInput {
  fn from(elem: MessageInput) -> Self {
    Self::V0(elem)
  }
}

struct MessageVersion(u8);

impl<'de> Deserialize<'de> for MessageVersion {
  fn deserialize<D>(deserializer: D) -> Result<MessageVersion, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct PrefixVisitor;

    impl<'de> Visitor<'de> for PrefixVisitor {
      type Value = MessageVersion;

      fn expecting(&self, formatter: &mut Formatter<'_>) -> core::fmt::Result {
        formatter.write_str("transaction message prefix")
      }

      fn visit_u8<E>(self, byte: u8) -> Result<MessageVersion, E>
      where
        E: de::Error,
      {
        if byte & MESSAGE_VERSION_PREFIX != 0 {
          Ok(MessageVersion(byte & !MESSAGE_VERSION_PREFIX))
        } else {
          Err(de::Error::custom(crate::Error::SolanaUnsupportedMessageFormat))
        }
      }
    }

    deserializer.deserialize_u8(PrefixVisitor)
  }
}
