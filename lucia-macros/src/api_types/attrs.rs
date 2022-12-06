use crate::transport_group::TransportGroup;
use syn::{Meta, MetaList, NestedMeta, Path};

#[derive(Debug)]
pub(crate) struct Attrs<'attrs> {
  pub(crate) pkgs_aux: Option<&'attrs Path>,
  pub(crate) transports: Vec<TransportGroup>,
}

impl<'attrs> TryFrom<&'attrs [NestedMeta]> for Attrs<'attrs> {
  type Error = crate::Error;

  fn try_from(from: &'attrs [NestedMeta]) -> Result<Self, Self::Error> {
    let mut pkgs_aux = None;
    let mut transports = Vec::new();
    for nested_meta in from {
      let meta_list = if let NestedMeta::Meta(Meta::List(ref elem)) = *nested_meta {
        elem
      } else {
        continue;
      };
      let first_meta_list_path_seg = if let Some(elem) = meta_list.path.segments.first() {
        elem
      } else {
        continue;
      };
      match first_meta_list_path_seg.ident.to_string().as_str() {
        "pkgs_aux" => {
          pkgs_aux = first_nested_meta_path(meta_list);
        }
        "transport" => {
          transports =
            meta_list.nested.iter().map(|elem| elem.try_into()).collect::<crate::Result<_>>()?;
        }
        _ => {}
      }
    }
    Ok(Self { pkgs_aux, transports })
  }
}

fn first_nested_meta_path(meta_list: &MetaList) -> Option<&Path> {
  let meta = if let Some(NestedMeta::Meta(elem)) = meta_list.nested.first() {
    elem
  } else {
    return None;
  };
  if let Meta::Path(ref elem) = *meta {
    Some(elem)
  } else {
    None
  }
}
