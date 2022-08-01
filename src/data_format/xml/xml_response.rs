use crate::{
  dnsn::{Deserialize, Serialize},
  utils::ByteBuffer,
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

impl<'de, D> Deserialize<'de, ()> for XmlResponse<D>
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
  use crate::{
    data_format::XmlResponse,
    dnsn::SerdeXmlRs,
    utils::{seq_visitor::_SeqVisitor, ByteBuffer},
  };
  use serde::de::Deserializer;

  impl<'de, D> crate::dnsn::Deserialize<'de, SerdeXmlRs> for XmlResponse<D>
  where
    D: serde::Deserialize<'de>,
  {
    #[inline]
    fn from_bytes(bytes: &'de [u8], _: &mut SerdeXmlRs) -> crate::Result<Self> {
      Ok(serde_xml_rs::from_reader(bytes)?)
    }

    #[inline]
    fn seq_from_bytes<F>(bytes: &'de [u8], _: &mut SerdeXmlRs, cb: F) -> crate::Result<()>
    where
      F: FnMut(Self) -> crate::Result<()>,
    {
      let mut de = serde_xml_rs::Deserializer::new_from_reader(bytes);
      Ok(de.deserialize_seq(_SeqVisitor::_new(cb))?)
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
