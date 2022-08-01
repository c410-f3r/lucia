/// Type that indicates the usage of the `miniserde` dependency.
#[cfg(feature = "miniserde")]
#[derive(Debug, Default)]
pub struct Miniserde {}

_impl_se_collections!(
  for Miniserde => miniserde::Serialize;

  slice: |this, bytes, _drsr| { miniserde_serialize(bytes, &this)?; }
  slice_ref: |this, bytes, _drsr| { miniserde_serialize(bytes, this)?; }
  vec: |this, bytes, _drsr| { miniserde_serialize(bytes, this)?; }
);

pub(crate) fn miniserde_deserialize<T>(bytes: &[u8]) -> crate::Result<T>
where
  T: miniserde::Deserialize,
{
  Ok(miniserde::json::from_str(core::str::from_utf8(bytes)?)?)
}

pub(crate) fn miniserde_deserialize_seq<F, T>(bytes: &[u8], mut cb: F) -> crate::Result<()>
where
  F: FnMut(T) -> crate::Result<()>,
  T: miniserde::Deserialize,
{
  let vec: Vec<T> = miniserde::json::from_str(core::str::from_utf8(bytes)?)?;
  for data in vec {
    cb(data)?
  }
  Ok(())
}

pub(crate) fn miniserde_serialize<B, T>(bytes: &mut B, elem: &T) -> crate::Result<()>
where
  B: crate::utils::ByteBuffer,
  T: miniserde::Serialize,
{
  let vec: Vec<u8> = miniserde::json::to_string(elem).into();
  bytes.extend(vec)?;
  Ok(())
}
