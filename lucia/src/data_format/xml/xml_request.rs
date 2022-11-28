use crate::{dnsn::Serialize, misc::ByteBuffer};

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
  fn to_bytes<BB>(&mut self, _: &mut BB, _: &mut ()) -> crate::Result<()>
  where
    BB: ByteBuffer,
  {
    Ok(())
  }
}

#[cfg(feature = "serde-xml-rs")]
mod serde_xml_rs {
  use crate::{data_format::XmlRequest, dnsn::SerdeXmlRs, misc::ByteBuffer};

  impl<D> crate::dnsn::Serialize<SerdeXmlRs> for XmlRequest<D>
  where
    D: serde::Serialize,
  {
    #[inline]
    fn to_bytes<BB>(&mut self, bytes: &mut BB, _: &mut SerdeXmlRs) -> crate::Result<()>
    where
      BB: ByteBuffer,
    {
      serde_xml_rs::to_writer(bytes, &self.data)?;
      Ok(())
    }
  }
}
