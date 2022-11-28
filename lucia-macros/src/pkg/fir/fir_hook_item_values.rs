macro_rules! create_fir_hook_item_values {
  (
    $struct:ident,
    $fn_call_idents:ident,
    $item:ident,
    $fn_name:expr,
    $ext_params:ident,
    $error:ident,
  ) => {
    use crate::{item_with_attr_span::ItemWithAttrSpan, pkg::misc::fn_arg_typed};
    use proc_macro2::TokenStream;
    use syn::{Item, ItemFn};

    pub(crate) struct $struct<'module> {
      pub(crate) $fn_call_idents: TokenStream,
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
          if item_fn.sig.ident != $fn_name {
            return None;
          }
          let mut iter = item_fn.sig.inputs.iter();
          let _ = fn_arg_typed(iter.next()?)?;
          let fbsiv_call_idents = if iter.next().and_then(|el| fn_arg_typed(el)).is_some() {
            quote::quote!(&mut self.params, $ext_params)
          } else {
            quote::quote!($ext_params)
          };
          if iter.next().is_some() {
            return None;
          }
          Some((fbsiv_call_idents, item_fn))
        };
        let ($fn_call_idents, $item) = fun().ok_or(crate::Error::$error(from.span))?;
        Ok(Self { $fn_call_idents, $item })
      }
    }
  };
}
