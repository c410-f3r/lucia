use crate::{dnsn::Serialize, utils::ByteBuffer};

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
#[doc = generic_data_format_doc!("JSON request")]
pub struct JsonRequest<D> {
  /// Actual data
  pub data: D,
}

impl<D> Serialize<()> for JsonRequest<D> {
  #[inline]
  fn to_bytes<B>(&self, _: &mut B, _: &mut ()) -> crate::Result<()>
  where
    B: ByteBuffer,
  {
    Ok(())
  }
}

#[cfg(feature = "miniserde")]
mod miniserde {
  use crate::{
    data_format::JsonRequest,
    dnsn::{miniserde_serialize, Miniserde},
    utils::ByteBuffer,
  };

  impl<D> crate::dnsn::Serialize<Miniserde> for JsonRequest<D>
  where
    D: miniserde::Serialize,
  {
    #[inline]
    fn to_bytes<B>(&self, bytes: &mut B, _: &mut Miniserde) -> crate::Result<()>
    where
      B: ByteBuffer,
    {
      miniserde_serialize(bytes, &self.data)
    }
  }
}

#[cfg(feature = "serde_json")]
mod serde_json {
  use crate::{data_format::JsonRequest, dnsn::SerdeJson, utils::ByteBuffer};

  impl<D> crate::dnsn::Serialize<SerdeJson> for JsonRequest<D>
  where
    D: serde::Serialize,
  {
    #[inline]
    fn to_bytes<B>(&self, bytes: &mut B, _: &mut SerdeJson) -> crate::Result<()>
    where
      B: ByteBuffer,
    {
      serde_json::to_writer(bytes, &self.data)?;
      Ok(())
    }
  }
}
