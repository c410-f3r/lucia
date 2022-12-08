#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
#[doc = generic_data_format_doc!("verbatim request")]
pub struct VerbatimRequest<D> {
  /// Actual data
  pub data: D,
}

mod unit {
  use crate::{data_format::VerbatimRequest, dnsn::Serialize, misc::ByteBuffer};

  impl<D> Serialize<()> for VerbatimRequest<D> {
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
  use crate::{
    data_format::VerbatimRequest,
    dnsn::{Rkyv, _InnerSerializer},
    misc::ByteBuffer,
  };

  impl<D> crate::dnsn::Serialize<Rkyv> for VerbatimRequest<D>
  where
    for<'any> D: rkyv::Serialize<_InnerSerializer<'any>>,
  {
    #[inline]
    fn to_bytes<BB>(&mut self, bytes: &mut BB, drsr: &mut Rkyv) -> crate::Result<()>
    where
      BB: ByteBuffer,
    {
      drsr._serialize(bytes, &self.data)?;
      Ok(())
    }
  }
}
