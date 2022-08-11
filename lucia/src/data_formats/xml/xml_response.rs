use crate::{
  dnsn::{Deserialize, Serialize},
  misc::ByteBuffer,
};

/// Any opaque and generic JSON response
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
#[doc = generic_data_format_doc!("XML response")]
pub struct XmlResponse<D> {
  /// Actual data
  pub data: D,
}

impl<D> Deserialize<()> for XmlResponse<D>
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

impl<D> Serialize<()> for XmlResponse<D> {
  #[inline]
  fn to_bytes<B>(&self, _: &mut B, _: &mut ()) -> crate::Result<()>
  where
    B: ByteBuffer,
  {
    Ok(())
  }
}

#[cfg(feature = "serde-xml-rs")]
mod serde_xml_rs {
  use core::fmt::Display;

  use crate::{
    data_formats::XmlResponse,
    dnsn::SerdeXmlRs,
    misc::{seq_visitor::_SeqVisitor, ByteBuffer},
  };
  use serde::de::Deserializer;

  impl<D> crate::dnsn::Deserialize<SerdeXmlRs> for XmlResponse<D>
  where
    D: for<'de> serde::Deserialize<'de>,
  {
    #[inline]
    fn from_bytes(bytes: &[u8], _: &mut SerdeXmlRs) -> crate::Result<Self> {
      Ok(serde_xml_rs::from_reader(bytes)?)
    }

    #[inline]
    fn seq_from_bytes<E>(
      bytes: &[u8],
      _: &mut SerdeXmlRs,
      mut cb: impl FnMut(Self) -> Result<(), E>,
    ) -> Result<(), E>
    where
      E: Display + From<crate::Error>,
    {
      let mut de = serde_xml_rs::Deserializer::new_from_reader(bytes);
      de.deserialize_seq(_SeqVisitor::_new(|data| cb(Self { data }))).map_err(Into::into)?;
      Ok(())
    }
  }

  impl<D> crate::dnsn::Serialize<SerdeXmlRs> for XmlResponse<D>
  where
    D: serde::Serialize,
  {
    #[inline]
    fn to_bytes<B>(&self, bytes: &mut B, _: &mut SerdeXmlRs) -> crate::Result<()>
    where
      B: ByteBuffer,
    {
      serde_xml_rs::to_writer(bytes, self)?;
      Ok(())
    }
  }
}
