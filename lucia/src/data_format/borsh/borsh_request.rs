#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
#[doc = generic_data_format_doc!("Borsh request")]
pub struct BorshRequest<D> {
  /// Actual data
  pub data: D,
}

mod unit {
  use crate::{data_format::BorshRequest, dnsn::Serialize, misc::ByteBuffer};

  impl<D> Serialize<()> for BorshRequest<D> {
    #[inline]
    fn to_bytes<BB>(&mut self, _: &mut BB, _: &mut ()) -> crate::Result<()>
    where
      BB: ByteBuffer,
    {
      Ok(())
    }
  }
}

#[cfg(feature = "borsh")]
mod borsh {
  use crate::{data_format::BorshRequest, dnsn::Borsh, misc::ByteBuffer};
  use borsh::BorshSerialize;

  impl<D> crate::dnsn::Serialize<Borsh> for BorshRequest<D>
  where
    D: BorshSerialize,
  {
    #[inline]
    fn to_bytes<BB>(&mut self, bytes: &mut BB, _: &mut Borsh) -> crate::Result<()>
    where
      BB: ByteBuffer,
    {
      self.data.serialize(bytes)?;
      Ok(())
    }
  }
}
