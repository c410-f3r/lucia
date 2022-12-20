/// GraphQL request or operation, can be a query or a mutation.
#[cfg_attr(feature = "miniserde", derive(miniserde::Serialize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug)]
pub struct GraphQlRequest<ON, Q, V> {
  /// Describes what type of operation you're intending to perform.
  #[cfg_attr(
    all(feature = "serde", not(feature = "miniserde")),
    serde(skip_serializing_if = "Option::is_none")
  )]
  pub operation_name: Option<ON>,
  /// Describes the desired data to be fetched.
  pub query: Q,
  /// Separated data intended to help queries.
  #[cfg_attr(
    all(feature = "serde", not(feature = "miniserde")),
    serde(skip_serializing_if = "Option::is_none")
  )]
  pub variables: Option<V>,
}

#[cfg(feature = "serde_json")]
mod serde_json {
  use crate::{data_format::GraphQlRequest, dnsn::SerdeJson, misc::ByteBuffer};

  impl<ON, Q, V> crate::dnsn::Serialize<SerdeJson> for GraphQlRequest<ON, Q, V>
  where
    ON: serde::Serialize,
    Q: serde::Serialize,
    V: serde::Serialize,
  {
    #[inline]
    fn to_bytes<BB>(&mut self, bytes: &mut BB, _: &mut SerdeJson) -> crate::Result<()>
    where
      BB: ByteBuffer,
    {
      if core::mem::size_of::<Self>() == 0 {
        return Ok(());
      }
      serde_json::to_writer(bytes, self)?;
      Ok(())
    }
  }
}

#[cfg(feature = "simd-json")]
mod simd_json {
  use crate::{data_format::GraphQlRequest, dnsn::SimdJson, misc::ByteBuffer};

  impl<ON, Q, V> crate::dnsn::Serialize<SimdJson> for GraphQlRequest<ON, Q, V>
  where
    ON: serde::Serialize,
    Q: serde::Serialize,
    V: serde::Serialize,
  {
    fn to_bytes<BB>(&mut self, bytes: &mut BB, _: &mut SimdJson) -> crate::Result<()>
    where
      BB: ByteBuffer,
    {
      if core::mem::size_of::<Self>() == 0 {
        return Ok(());
      }
      simd_json::to_writer(bytes, self)?;
      Ok(())
    }
  }
}
