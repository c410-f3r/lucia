#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
#[doc = generic_data_format_doc!("Protobuf response")]
pub struct ProtobufResponse<D> {
  /// Actual data
  pub data: D,
}

mod unit {
  use crate::{
    data_format::ProtobufResponse,
    dnsn::{Deserialize, Serialize},
    misc::ByteBuffer,
  };

  impl<D> Deserialize<()> for ProtobufResponse<D>
  where
    D: Default,
  {
    #[inline]
    fn from_bytes(_: &[u8], _: &mut ()) -> crate::Result<Self> {
      Ok(Self { data: D::default() })
    }

    #[inline]
    fn seq_from_bytes<E>(
      _: &[u8],
      _: &mut (),
      _: impl FnMut(Self) -> Result<(), E>,
    ) -> Result<(), E>
    where
      E: From<crate::Error>,
    {
      Ok(())
    }
  }

  impl<D> Serialize<()> for ProtobufResponse<D> {
    #[inline]
    fn to_bytes<BB>(&mut self, _: &mut BB, _: &mut ()) -> crate::Result<()>
    where
      BB: ByteBuffer,
    {
      Ok(())
    }
  }
}

#[cfg(feature = "protobuf")]
mod protobuf {
  use protobuf::Message;

  use crate::{data_format::ProtobufResponse, dnsn::Protobuf};
  use core::fmt::Display;

  impl<D> crate::dnsn::Deserialize<Protobuf> for ProtobufResponse<D>
  where
    D: Message,
  {
    #[inline]
    fn from_bytes(bytes: &[u8], _: &mut Protobuf) -> crate::Result<Self> {
      Ok(Self { data: Message::parse_from_bytes(bytes)? })
    }

    #[inline]
    fn seq_from_bytes<E>(
      _: &[u8],
      _: &mut Protobuf,
      _: impl FnMut(Self) -> Result<(), E>,
    ) -> Result<(), E>
    where
      E: Display + From<crate::Error>,
    {
      Err(crate::Error::UnsupportedOperation.into())
    }
  }
}
