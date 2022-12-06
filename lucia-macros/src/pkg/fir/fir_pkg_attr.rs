use syn::{punctuated::Punctuated, Meta, MetaList, NestedMeta, Path, Token};

const EMPTY_NESTED_META: &Punctuated<NestedMeta, Token![,]> = &Punctuated::new();

#[derive(Debug)]
pub(crate) struct FirPkgAttr<'attrs> {
  pub(crate) api: &'attrs Path,
  pub(crate) data_formats: &'attrs Punctuated<NestedMeta, Token![,]>,
  pub(crate) error: Option<&'attrs Path>,
  pub(crate) transports: &'attrs Punctuated<NestedMeta, Token![,]>,
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
      match first_meta_list_path_seg.ident.to_string().as_str() {
        "api" => {
          api = first_nested_meta_path(meta_list);
        }
        "data_format" => {
          data_formats = Some(&meta_list.nested);
        }
        "error" => {
          error = first_nested_meta_path(meta_list);
        }
        "transport" => {
          transports = Some(&meta_list.nested);
        }
        _ => {}
      }
    }
    Ok(Self {
      api: api.ok_or(crate::Error::MandatoryOuterAttrsAreNotPresent)?,
      data_formats: data_formats.unwrap_or(EMPTY_NESTED_META),
      error,
      transports: transports.unwrap_or(EMPTY_NESTED_META),
    })
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
