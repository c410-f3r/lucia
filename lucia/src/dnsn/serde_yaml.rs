/// Type that indicates the usage of the `serde-xml-rs` dependency.
#[derive(Debug)]
pub struct SerdeYaml;

_impl_se_collections!(
  for SerdeYaml => serde::Serialize;

  array: |this, bytes, _drsr| { serde_yaml::to_writer(bytes, &this[..])?; }
  arrayvec: |this, bytes, _drsr| { serde_yaml::to_writer(bytes, this)?; }
  slice_ref: |this, bytes, _drsr| { serde_yaml::to_writer(bytes, this)?; }
  vec: |this, bytes, _drsr| { serde_yaml::to_writer(bytes, this)?; }
);
