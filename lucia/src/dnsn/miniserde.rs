/// Type that indicates the usage of the `miniserde` dependency.
#[cfg(feature = "miniserde")]
#[derive(Debug, Default)]
pub struct Miniserde;

_impl_se_collections!(
  for Miniserde => miniserde::Serialize;

  slice_ref: |this, bytes, _drsr| { miniserde_serialize(bytes, this)?; }
  vec: |this, bytes, _drsr| { miniserde_serialize(bytes, this)?; }
);

pub(crate) fn miniserde_serialize<B, T>(bytes: &mut B, elem: &T) -> crate::Result<()>
where
  B: crate::misc::ByteBuffer,
  T: miniserde::Serialize,
{
  let vec: Vec<u8> = miniserde::json::to_string(elem).into();
  bytes.extend(vec)?;
  Ok(())
}
