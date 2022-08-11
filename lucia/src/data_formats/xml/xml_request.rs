use crate::{
  dnsn::Serialize,
  misc::{ByteBuffer, FromErrorTy},
  req_res::{Request, RequestParamsModifier},
};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
#[doc = generic_data_format_doc!("XML request")]
pub struct XmlRequest<D> {
  /// Actual data
  pub data: D,
}

impl<D> FromErrorTy for XmlRequest<D>
where
  D: FromErrorTy,
{
  type Error = D::Error;
}

impl<CP, D, DRSR, REQP, RESP> Request<CP, DRSR, REQP, RESP> for XmlRequest<D>
where
  D: Request<CP, DRSR, REQP, RESP>,
{
  type Data = D::Data;
  type ProcessedResponse = D::ProcessedResponse;
  type RawResponse = D::RawResponse;

  #[inline]
  fn data(&self) -> &Self::Data {
    self.data.data()
  }

  #[inline]
  fn process(raw: Self::RawResponse, resp: &RESP) -> Result<Self::ProcessedResponse, Self::Error> {
    D::process(raw, resp)
  }
}

impl<CP, REQP, D> RequestParamsModifier<CP, REQP> for XmlRequest<D>
where
  D: RequestParamsModifier<CP, REQP>,
{
  #[inline]
  fn modify_all_params(cp: &mut CP, reqp: REQP) -> Result<(), Self::Error> {
    D::modify_all_params(cp, reqp)
  }
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
  use crate::{data_formats::XmlRequest, dnsn::SerdeXmlRs, misc::ByteBuffer};

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
