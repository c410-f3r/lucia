/// Type that indicates the usage of the `miniserde` dependency.
#[derive(Debug)]
pub struct Miniserde;

_impl_se_collections!(
  for Miniserde => miniserde::Serialize;

  slice_ref: |this, bytes, _drsr| { miniserde_serialize(bytes, this)?; }
  vec: |this, bytes, _drsr| { miniserde_serialize(bytes, this)?; }
);

pub(crate) fn miniserde_serialize<BB, E>(bytes: &mut BB, elem: &E) -> crate::Result<()>
where
  BB: crate::misc::ByteBuffer,
  E: miniserde::Serialize,
{
  let vec: Vec<u8> = miniserde::json::to_string(elem).into();
  bytes.extend(vec)?;
  Ok(())
}
