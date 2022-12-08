#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
#[doc = generic_data_format_doc!("Borsh response")]
pub struct BorshResponse<D> {
  /// Actual data
  pub data: D,
}

mod unit {
  use crate::{
    data_format::BorshResponse,
    dnsn::{Deserialize, Serialize},
    misc::ByteBuffer,
  };

  impl<D> Deserialize<()> for BorshResponse<D>
  where
    D: Default,
  {
    #[inline]
    fn from_bytes(_: &[u8], _: &mut ()) -> crate::Result<Self> {
      Ok(Self { data: D::default() })
    }

    #[inline]
    fn seq_from_bytes<E>(
      _: &[u8],
      _: &mut (),
      _: impl FnMut(Self) -> Result<(), E>,
    ) -> Result<(), E>
    where
      E: From<crate::Error>,
    {
      Ok(())
    }
  }

  impl<D> Serialize<()> for BorshResponse<D> {
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
  use crate::{data_format::BorshResponse, dnsn::Borsh};
  use borsh::BorshDeserialize;
  use core::fmt::Display;

  impl<D> crate::dnsn::Deserialize<Borsh> for BorshResponse<D>
  where
    D: BorshDeserialize,
  {
    #[inline]
    fn from_bytes(mut bytes: &[u8], _: &mut Borsh) -> crate::Result<Self> {
      Ok(Self { data: D::deserialize(&mut bytes)? })
    }

    #[inline]
    fn seq_from_bytes<E>(
      _: &[u8],
      _: &mut Borsh,
      _: impl FnMut(Self) -> Result<(), E>,
    ) -> Result<(), E>
    where
      E: Display + From<crate::Error>,
    {
      Err(crate::Error::UnsupportedOperation.into())
    }
  }
}
