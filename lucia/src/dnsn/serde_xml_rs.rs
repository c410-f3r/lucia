/// Type that indicates the usage of the `serde-xml-rs` dependency.
#[derive(Debug)]
pub struct SerdeXmlRs;

_impl_se_collections!(
  for SerdeXmlRs => serde::Serialize;

  array: |this, bytes, _drsr| { serde_xml_rs::to_writer(bytes, &&this[..])?; }
  arrayvec: |this, bytes, _drsr| { serde_xml_rs::to_writer(bytes, this)?; }
  slice_ref: |this, bytes, _drsr| { serde_xml_rs::to_writer(bytes, this)?; }
  vec: |this, bytes, _drsr| { serde_xml_rs::to_writer(bytes, this)?; }
);
