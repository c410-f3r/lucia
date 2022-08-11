use crate::{
  dnsn::{Deserialize, Serialize},
  misc::ByteBuffer,
};

#[doc = generic_data_format_doc!("JSON response")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct JsonResponse<D> {
  /// Actual data
  pub data: D,
}

impl<D> Deserialize<()> for JsonResponse<D>
where
  D: Default,
{
  #[inline]
  fn from_bytes(_: &[u8], _: &mut ()) -> crate::Result<Self> {
    Ok(Self { data: D::default() })
  }

  #[inline]
  fn seq_from_bytes<E>(_: &[u8], _: &mut (), _: impl FnMut(Self) -> Result<(), E>) -> Result<(), E>
  where
    E: From<crate::Error>,
  {
    Ok(())
  }
}

impl<D> Serialize<()> for JsonResponse<D> {
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
  use core::{fmt::Display, str};

  use crate::{
    data_formats::JsonResponse,
    dnsn::{miniserde_serialize, Miniserde},
    misc::ByteBuffer,
  };

  impl<D> crate::dnsn::Deserialize<Miniserde> for JsonResponse<D>
  where
    D: miniserde::Deserialize,
  {
    #[inline]
    fn from_bytes(bytes: &[u8], _: &mut Miniserde) -> crate::Result<Self> {
      Ok(Self { data: miniserde::json::from_str(core::str::from_utf8(bytes)?)? })
    }

    #[inline]
    fn seq_from_bytes<E>(
      bytes: &[u8],
      _: &mut Miniserde,
      mut cb: impl FnMut(Self) -> Result<(), E>,
    ) -> Result<(), E>
    where
      E: Display + From<crate::Error>,
    {
      let vec = || crate::Result::Ok(miniserde::json::from_str(str::from_utf8(bytes)?)?);
      for data in vec() {
        cb(Self { data })?
      }
      Ok(())
    }
  }

  impl<D> crate::dnsn::Serialize<Miniserde> for JsonResponse<D>
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
  use core::fmt::Display;

  use crate::{
    data_formats::JsonResponse,
    dnsn::SerdeJson,
    misc::{seq_visitor::_SeqVisitor, ByteBuffer},
  };
  use serde::de::Deserializer;

  impl<D> crate::dnsn::Deserialize<SerdeJson> for JsonResponse<D>
  where
    D: for<'de> serde::Deserialize<'de>,
  {
    #[inline]
    fn from_bytes(bytes: &[u8], _: &mut SerdeJson) -> crate::Result<Self> {
      Ok(JsonResponse { data: serde_json::from_slice(bytes)? })
    }

    #[inline]
    fn seq_from_bytes<E>(
      bytes: &[u8],
      _: &mut SerdeJson,
      mut cb: impl FnMut(Self) -> Result<(), E>,
    ) -> Result<(), E>
    where
      E: Display + From<crate::Error>,
    {
      let mut de = serde_json::Deserializer::from_slice(bytes);
      de.deserialize_seq(_SeqVisitor::_new(|data| cb(Self { data }))).map_err(Into::into)?;
      Ok(())
    }
  }

  impl<D> crate::dnsn::Serialize<SerdeJson> for JsonResponse<D>
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
