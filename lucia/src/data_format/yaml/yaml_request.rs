use crate::{dnsn::Serialize, misc::ByteBuffer};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
#[doc = generic_data_format_doc!("YAML request")]
pub struct YamlRequest<D> {
  /// Actual data
  pub data: D,
}

impl<D> Serialize<()> for YamlRequest<D> {
  #[inline]
  fn to_bytes<BB>(&mut self, _: &mut BB, _: &mut ()) -> crate::Result<()>
  where
    BB: ByteBuffer,
  {
    Ok(())
  }
}

#[cfg(feature = "serde_yaml")]
mod serde_yaml {
  use crate::{data_format::YamlRequest, dnsn::SerdeYaml, misc::ByteBuffer};

  impl<D> crate::dnsn::Serialize<SerdeYaml> for YamlRequest<D>
  where
    D: serde::Serialize,
  {
    #[inline]
    fn to_bytes<BB>(&mut self, bytes: &mut BB, _: &mut SerdeYaml) -> crate::Result<()>
    where
      BB: ByteBuffer,
    {
      if core::mem::size_of::<D>() == 0 {
        return Ok(());
      }
      serde_yaml::to_writer(bytes, &self.data)?;
      Ok(())
    }
  }
}
