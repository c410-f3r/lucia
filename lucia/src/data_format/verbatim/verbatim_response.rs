#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
#[doc = generic_data_format_doc!("verbatim response")]
pub struct VerbatimResponse<D> {
  /// Actual data
  pub data: D,
}

mod unit {
  use crate::{
    data_format::VerbatimResponse,
    dnsn::{Deserialize, Serialize},
    misc::ByteBuffer,
  };

  impl<D> Deserialize<()> for VerbatimResponse<D>
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

  impl<D> Serialize<()> for VerbatimResponse<D> {
    #[inline]
    fn to_bytes<BB>(&mut self, _: &mut BB, _: &mut ()) -> crate::Result<()>
    where
      BB: ByteBuffer,
    {
      Ok(())
    }
  }
}

#[cfg(feature = "rkyv")]
mod rkyv {
  use crate::{data_format::VerbatimResponse, dnsn::Rkyv};
  use core::fmt::Display;
  use rkyv::{
    bytecheck::CheckBytes, de::deserializers::SharedDeserializeMap,
    validation::validators::DefaultValidator, Archive,
  };

  impl<D> crate::dnsn::Deserialize<Rkyv> for VerbatimResponse<D>
  where
    D: Archive,
    for<'any> D::Archived:
      CheckBytes<DefaultValidator<'any>> + rkyv::Deserialize<D, SharedDeserializeMap>,
  {
    fn from_bytes(bytes: &[u8], _: &mut Rkyv) -> crate::Result<Self> {
      Ok(Self {
        data: rkyv::from_bytes(bytes)
          .map_err(|_err| crate::Error::RkyvDer(core::any::type_name::<D>()))?,
      })
    }

    fn seq_from_bytes<E>(
      _: &[u8],
      _: &mut Rkyv,
      _: impl FnMut(Self) -> Result<(), E>,
    ) -> Result<(), E>
    where
      E: Display + From<crate::Error>,
    {
      Err(crate::Error::UnsupportedOperation.into())
    }
  }
}
