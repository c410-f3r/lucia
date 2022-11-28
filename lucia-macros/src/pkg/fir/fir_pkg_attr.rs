use syn::{punctuated::Punctuated, Meta, MetaList, NestedMeta, Path, Token};

#[derive(Debug)]
pub(crate) struct FirPkgAttr<'attrs> {
  pub(crate) api: &'attrs Path,
  pub(crate) data_formats: Option<&'attrs Punctuated<NestedMeta, Token![,]>>,
  pub(crate) error: Option<&'attrs Path>,
  pub(crate) transports: Option<&'attrs Punctuated<NestedMeta, Token![,]>>,
}

impl<'attrs> TryFrom<&'attrs [NestedMeta]> for FirPkgAttr<'attrs> {
  type Error = crate::Error;

  fn try_from(from: &'attrs [NestedMeta]) -> Result<Self, Self::Error> {
    let mut api = None;
    let mut data_formats = None;
    let mut error = None;
    let mut transports = None;
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
      if first_meta_list_path_seg.ident == "api" {
        api = first_nested_meta_path(meta_list);
      } else if first_meta_list_path_seg.ident == "data_format" {
        data_formats = Some(&meta_list.nested);
      } else if first_meta_list_path_seg.ident == "error" {
        error = first_nested_meta_path(meta_list);
      } else if first_meta_list_path_seg.ident == "transport" {
        transports = Some(&meta_list.nested);
      } else {
      }
    }
    Ok(Self {
      api: api.ok_or(crate::Error::MandatoryOuterAttrsAreNotPresent)?,
      data_formats,
      error,
      transports,
    })
  }
}

fn first_nested_meta_path(meta_list: &MetaList) -> Option<&Path> {
  let meta = if let Some(&NestedMeta::Meta(ref elem)) = meta_list.nested.first() {
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
