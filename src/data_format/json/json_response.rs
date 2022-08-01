use crate::{
  dnsn::{Deserialize, Serialize},
  utils::ByteBuffer,
};

#[doc = generic_data_format_doc!("JSON response")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct JsonResponse<D> {
  /// Actual data
  pub data: D,
}

impl<'de, D> Deserialize<'de, ()> for JsonResponse<D>
where
  D: Default,
{
  #[inline]
  fn from_bytes(_: &'de [u8], _: &mut ()) -> crate::Result<Self> {
    Ok(Self { data: D::default() })
  }

  #[inline]
  fn seq_from_bytes<F>(_: &'de [u8], _: &mut (), _: F) -> crate::Result<()>
  where
    F: FnMut(Self) -> crate::Result<()>,
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
  use crate::{
    data_format::JsonResponse,
    dnsn::{miniserde_deserialize, miniserde_deserialize_seq, miniserde_serialize, Miniserde},
    utils::ByteBuffer,
  };

  impl<'de, D> crate::dnsn::Deserialize<'de, Miniserde> for JsonResponse<D>
  where
    D: miniserde::Deserialize,
  {
    #[inline]
    fn from_bytes(bytes: &'de [u8], _: &mut Miniserde) -> crate::Result<Self> {
      Ok(Self { data: miniserde_deserialize(bytes)? })
    }

    #[inline]
    fn seq_from_bytes<F>(bytes: &'de [u8], _: &mut Miniserde, mut cb: F) -> crate::Result<()>
    where
      F: FnMut(Self) -> crate::Result<()>,
    {
      miniserde_deserialize_seq(bytes, |data| cb(Self { data }))
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
  use crate::{
    data_format::JsonResponse,
    dnsn::SerdeJson,
    utils::{seq_visitor::_SeqVisitor, ByteBuffer},
  };
  use serde::de::Deserializer;

  impl<'de, D> crate::dnsn::Deserialize<'de, SerdeJson> for JsonResponse<D>
  where
    D: serde::Deserialize<'de>,
  {
    #[inline]
    fn from_bytes(bytes: &'de [u8], _: &mut SerdeJson) -> crate::Result<Self> {
      Ok(JsonResponse { data: serde_json::from_slice(bytes)? })
    }

    #[inline]
    fn seq_from_bytes<F>(bytes: &'de [u8], _: &mut SerdeJson, mut cb: F) -> crate::Result<()>
    where
      F: FnMut(Self) -> crate::Result<()>,
    {
      let mut de = serde_json::Deserializer::from_slice(bytes);
      Ok(de.deserialize_seq(_SeqVisitor::_new(|data| {
        cb(JsonResponse { data })?;
        Ok(())
      }))?)
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
