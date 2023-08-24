macro_rules! create_fir_hook_item_values {
  (
    $struct:ident,
    $fn_call_idents:ident,
    $item:ident,
    $fn_name:expr,
    $fn_args_idents:expr,
    $error:ident,
  ) => {
    use crate::item_with_attr_span::ItemWithAttrSpan;
    use proc_macro2::TokenStream;
    use syn::{punctuated::Punctuated, FnArg, Item, ItemFn, Pat, Token};

    pub(crate) struct $struct<'module> {
  pub(crate) $fn_call_idents: Punctuated<TokenStream, Token![,]>,
      pub(crate) $item: &'module ItemFn,
    }

    impl<'others> TryFrom<ItemWithAttrSpan<(), &'others Item>> for $struct<'others> {
      type Error = crate::Error;

      fn try_from(from: ItemWithAttrSpan<(), &'others Item>) -> Result<Self, Self::Error> {
        let fun = || {
          let item_fn = match *from.item {
            Item::Fn(ref item_fn) => item_fn,
            Item::Const(_)
            | Item::Enum(_)
            | Item::ExternCrate(_)
            | Item::ForeignMod(_)
            | Item::Impl(_)
            | Item::Macro(_)
            | Item::Macro2(_)
            | Item::Mod(_)
            | Item::Static(_)
            | Item::Struct(_)
            | Item::Trait(_)
            | Item::TraitAlias(_)
            | Item::Type(_)
            | Item::Union(_)
            | Item::Use(_)
            | Item::Verbatim(_)
            | _ => return None,
          };
          let call_idents_cb: fn(&str) -> Option<TokenStream> = $fn_args_idents;
          let mut call_idents = Punctuated::new();
          for fn_arg in &item_fn.sig.inputs {
            let FnArg::Typed(ref pat_type) = *fn_arg else {
              continue;
            };
            let Pat::Ident(ref pat_ident) = *pat_type.pat else {
              continue;
            };
            let tt = call_idents_cb(pat_ident.ident.to_string().as_str())?;
            call_idents.push(tt);
          }
          Some((call_idents, item_fn))
        };
        let ($fn_call_idents, $item) = fun().ok_or(crate::Error::$error(from.span))?;
        Ok(Self { $fn_call_idents, $item })
      }
    }
  };
}
