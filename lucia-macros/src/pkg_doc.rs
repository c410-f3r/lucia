use crate::misc::{push_allow_missing_docs, push_docs};
use proc_macro2::Span;
use quote::ToTokens;
use syn::{Attribute, Fields, Item};

const CONTAINER_DOC: &str = "\
  Please see the official API's documentation to get more information about this request. If \
  it is unknown to you, an URL is avaiable at the module's top-level documentation.
  \n\
  `lucia` is a coding framework that usually follows an external specification. As such, \
  it probably would be very hard to manage and synchronize two separated copies.\
";

pub(crate) fn pkg_doc(
  _: proc_macro::TokenStream,
  item: proc_macro::TokenStream,
) -> crate::Result<proc_macro::TokenStream> {
  let mut item: Item = syn::parse_macro_input::parse(item)?;

  match item {
    Item::Enum(ref mut container) => {
      manage_container_doc(&mut container.attrs, container.ident.span())?;
      for variant in container.variants.iter_mut() {
        manage_attrs_doc(&mut variant.attrs)
      }
    }
    Item::Struct(ref mut container) => {
      manage_container_doc(&mut container.attrs, container.ident.span())?;
      match container.fields {
        Fields::Named(ref mut elem) => {
          for variant in elem.named.iter_mut() {
            manage_attrs_doc(&mut variant.attrs)
          }
        }
        Fields::Unnamed(ref mut elem) => {
          for variant in elem.unnamed.iter_mut() {
            manage_attrs_doc(&mut variant.attrs)
          }
        }
        Fields::Unit => {}
      }
    }
    Item::Type(ref mut container) => {
      manage_container_doc(&mut container.attrs, container.ident.span())?;
    }
    Item::Const(_)
    | Item::ExternCrate(_)
    | Item::Fn(_)
    | Item::ForeignMod(_)
    | Item::Impl(_)
    | Item::Macro(_)
    | Item::Macro2(_)
    | Item::Mod(_)
    | Item::Static(_)
    | Item::Trait(_)
    | Item::TraitAlias(_)
    | Item::Union(_)
    | Item::Use(_)
    | Item::Verbatim(_)
    | _ => {}
  }

  Ok(item.to_token_stream().into())
}

pub(crate) fn has_at_least_one_doc(attrs: &[Attribute]) -> bool {
  attrs.iter().any(|attr| {
    if let Some(last) = attr.path.segments.last() {
      last.ident == "doc"
    } else {
      false
    }
  })
}

pub(crate) fn manage_attrs_doc(attrs: &mut Vec<Attribute>) {
  if !has_at_least_one_doc(attrs) {
    push_allow_missing_docs(attrs);
  }
}

pub(crate) fn manage_container_doc(attrs: &mut Vec<Attribute>, span: Span) -> crate::Result<()> {
  if has_at_least_one_doc(attrs) {
    Err(crate::Error::ElemHasDoc(span))
  } else {
    push_docs(attrs, CONTAINER_DOC);
    Ok(())
  }
}