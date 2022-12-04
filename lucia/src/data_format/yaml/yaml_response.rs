use crate::{
  dnsn::{Deserialize, Serialize},
  misc::ByteBuffer,
};

/// Any opaque and generic JSON response
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
#[doc = generic_data_format_doc!("YAML response")]
pub struct YamlResponse<D> {
  /// Actual data
  pub data: D,
}

impl<D> Deserialize<()> for YamlResponse<D>
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

impl<D> Serialize<()> for YamlResponse<D> {
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
  use crate::{
    data_format::YamlResponse,
    dnsn::SerdeYaml,
    misc::{seq_visitor::_SeqVisitor, ByteBuffer},
  };
  use core::fmt::Display;
  use serde::de::Deserializer;

  impl<D> crate::dnsn::Deserialize<SerdeYaml> for YamlResponse<D>
  where
    D: for<'de> serde::Deserialize<'de>,
  {
    #[inline]
    fn from_bytes(bytes: &[u8], _: &mut SerdeYaml) -> crate::Result<Self> {
      Ok(serde_yaml::from_reader(bytes)?)
    }

    #[inline]
    fn seq_from_bytes<E>(
      bytes: &[u8],
      _: &mut SerdeYaml,
      mut cb: impl FnMut(Self) -> Result<(), E>,
    ) -> Result<(), E>
    where
      E: Display + From<crate::Error>,
    {
      let de = serde_yaml::Deserializer::from_reader(bytes);
      de.deserialize_seq(_SeqVisitor::_new(|data| cb(Self { data }))).map_err(Into::into)?;
      Ok(())
    }
  }

  impl<D> crate::dnsn::Serialize<SerdeYaml> for YamlResponse<D>
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
      serde_yaml::to_writer(bytes, self)?;
      Ok(())
    }
  }
}
