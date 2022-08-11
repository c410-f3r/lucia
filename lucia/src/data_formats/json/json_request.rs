use crate::{
  dnsn::Serialize,
  misc::{ByteBuffer, FromErrorTy},
  req_res::{Request, RequestParamsModifier},
};

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
#[doc = generic_data_format_doc!("JSON request")]
pub struct JsonRequest<D> {
  /// Actual data
  pub data: D,
}

impl<D> FromErrorTy for JsonRequest<D>
where
  D: FromErrorTy,
{
  type Error = D::Error;
}

impl<CP, D, DRSR, REQP, RESP> Request<CP, DRSR, REQP, RESP> for JsonRequest<D>
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

impl<CP, REQP, D> RequestParamsModifier<CP, REQP> for JsonRequest<D>
where
  D: RequestParamsModifier<CP, REQP>,
{
  #[inline]
  fn modify_all_params(cp: &mut CP, reqp: REQP) -> Result<(), Self::Error> {
    D::modify_all_params(cp, reqp)
  }
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
    data_formats::JsonRequest,
    dnsn::{miniserde_serialize, Miniserde},
    misc::ByteBuffer,
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
  use crate::{data_formats::JsonRequest, dnsn::SerdeJson, misc::ByteBuffer};

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
