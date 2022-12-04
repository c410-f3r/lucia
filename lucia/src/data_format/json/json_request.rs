use crate::{dnsn::Serialize, misc::ByteBuffer};

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
#[doc = generic_data_format_doc!("JSON request")]
pub struct JsonRequest<D> {
  /// Actual data
  pub data: D,
}

impl<D> Serialize<()> for JsonRequest<D> {
  #[inline]
  fn to_bytes<BB>(&mut self, _: &mut BB, _: &mut ()) -> crate::Result<()>
  where
    BB: ByteBuffer,
  {
    Ok(())
  }
}

#[cfg(feature = "miniserde")]
mod miniserde {
  use crate::{
    data_format::JsonRequest,
    dnsn::{miniserde_serialize, Miniserde},
    misc::ByteBuffer,
  };

  impl<D> crate::dnsn::Serialize<Miniserde> for JsonRequest<D>
  where
    D: miniserde::Serialize,
  {
    #[inline]
    fn to_bytes<BB>(&mut self, bytes: &mut BB, _: &mut Miniserde) -> crate::Result<()>
    where
      BB: ByteBuffer,
    {
      if core::mem::size_of::<D>() == 0 {
        return Ok(());
      }
      miniserde_serialize(bytes, &self.data)
    }
  }
}

#[cfg(feature = "serde_json")]
mod serde_json {
  use crate::{data_format::JsonRequest, dnsn::SerdeJson, misc::ByteBuffer};

  impl<D> crate::dnsn::Serialize<SerdeJson> for JsonRequest<D>
  where
    D: serde::Serialize,
  {
    #[inline]
    fn to_bytes<BB>(&mut self, bytes: &mut BB, _: &mut SerdeJson) -> crate::Result<()>
    where
      BB: ByteBuffer,
    {
      if core::mem::size_of::<D>() == 0 {
        return Ok(());
      }
      serde_json::to_writer(bytes, &self.data)?;
      Ok(())
    }
  }
}
