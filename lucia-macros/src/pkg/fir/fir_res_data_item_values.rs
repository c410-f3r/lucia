use crate::item_with_attr_span::ItemWithAttrSpan;
use proc_macro2::Ident;
use syn::Item;

#[derive(Debug)]
pub(crate) struct FirResDataItemValues<'module> {
  pub(crate) res_ident: &'module Ident,
}

impl<'module> TryFrom<ItemWithAttrSpan<(), &'module mut Item>> for FirResDataItemValues<'module> {
  type Error = crate::Error;

  fn try_from(from: ItemWithAttrSpan<(), &'module mut Item>) -> Result<Self, Self::Error> {
    let (res_ident, generics) = match *from.item {
      Item::Enum(ref mut item) => (&item.ident, &item.generics),
      Item::Struct(ref mut item) => (&item.ident, &item.generics),
      Item::Type(ref mut item) => (&item.ident, &item.generics),
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
      | _ => return Err(crate::Error::NoEnumStructOrType(from.span)),
    };
    if !res_ident.to_string().ends_with("ResData") {
      return Err(crate::Error::BadResData(res_ident.span()));
    }
    if !generics.params.is_empty() {
      return Err(crate::Error::ResponsesCanNotHaveTypeParams(res_ident.span()));
    }
    Ok(Self { res_ident })
  }
}
