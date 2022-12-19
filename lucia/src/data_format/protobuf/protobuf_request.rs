#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
#[doc = generic_data_format_doc!("Protobuf request")]
pub struct ProtobufRequest<D> {
  /// Actual data
  pub data: D,
}

mod unit {
  use crate::{data_format::ProtobufRequest, dnsn::Serialize, misc::ByteBuffer};

  impl<D> Serialize<()> for ProtobufRequest<D> {
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
  use crate::{data_format::ProtobufRequest, dnsn::Protobuf, misc::ByteBuffer};
  use protobuf::Message;

  impl<D> crate::dnsn::Serialize<Protobuf> for ProtobufRequest<D>
  where
    D: Message,
  {
    #[inline]
    fn to_bytes<BB>(&mut self, bytes: &mut BB, _: &mut Protobuf) -> crate::Result<()>
    where
      BB: ByteBuffer,
    {
      self.data.write_to_writer(bytes)?;
      Ok(())
    }
  }
}
