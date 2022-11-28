/// Type that indicates the usage of the `serde_json` dependency.
#[derive(Debug)]
pub struct SerdeJson;

_impl_se_collections!(
  for SerdeJson => serde::Serialize;

  array: |this, bytes, _drsr| { serde_json::to_writer(bytes, &this[..])?; }
  arrayvec: |this, bytes, _drsr| { serde_json::to_writer(bytes, this)?; }
  slice_ref: |this, bytes, _drsr| { serde_json::to_writer(bytes, this)?; }
  vec: |this, bytes, _drsr| { serde_json::to_writer(bytes, this)?; }
);