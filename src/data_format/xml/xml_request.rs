use crate::{dnsn::Serialize, utils::ByteBuffer};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
#[doc = generic_data_format_doc!("XML request")]
pub struct XmlRequest<D> {
  /// Actual data
  pub data: D,
}

impl<D> Serialize<()> for XmlRequest<D> {
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
  use crate::{data_format::XmlRequest, dnsn::SerdeXmlRs, utils::ByteBuffer};

  impl<D> crate::dnsn::Serialize<SerdeXmlRs> for XmlRequest<D>
  where
    D: serde::Serialize,
  {
    #[inline]
    fn to_bytes<B>(&self, bytes: &mut B, _: &mut SerdeXmlRs) -> crate::Result<()>
    where
      B: ByteBuffer,
    {
      serde_xml_rs::to_writer(bytes, &self.data)?;
      Ok(())
    }
  }
}
