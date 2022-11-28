mod data_format;
mod data_format_elems;
mod enum_struct_or_type;
mod fir;
mod keywords;
mod misc;
mod sir;
mod transport_group;

use fir::{
  fir_aux_item_values::FirAuxItemValues,
  fir_before_sending_item_values::FirBeforeSendingItemValues, fir_items_values::FirItemsValues,
  fir_params_items_values::FirParamsItemValues, fir_pkg_attr::FirPkgAttr,
  fir_req_data_item_values::FirReqDataItemValues, fir_res_data_item_values::FirResDataItemValues,
};
use proc_macro2::{Ident, Span};
use quote::ToTokens;
use sir::{sir_final_values::SirFinalValues, sir_pkg_attr::SirEndpointAttr};
use syn::{
  parse_macro_input,
  punctuated::Punctuated,
  token::{Eq, Pub, Semi, Type},
  AttributeArgs, Generics, Item, ItemMod, ItemType, VisPublic, Visibility,
};

use crate::{
  item_with_attr_span::ItemWithAttrSpan,
  pkg::{fir::fir_after_sending_item_values::FirAfterSendingItemValues, misc::unit_type},
};

pub(crate) fn pkg(
  attrs: proc_macro::TokenStream,
  item: proc_macro::TokenStream,
) -> crate::Result<proc_macro::TokenStream> {
  let attr_args = parse_macro_input::parse::<AttributeArgs>(attrs)?;
  let mut item_mod: ItemMod = parse_macro_input::parse(item)?;
  let fiv = FirItemsValues::try_from(&mut item_mod)?;
  let freqdiv = FirReqDataItemValues::try_from(fiv.req_data)?;
  let mut camel_case_id = {
    let mut string = freqdiv.freqdiv_ident.to_string();
    if let Some(idx) = string.rfind("ReqData") {
      string.truncate(idx);
    }
    string
  };
  let mut params_item_unit_opt = None;
  let fpiv = if let Some(elem) = fiv.params {
    FirParamsItemValues::try_from(elem)?
  } else {
    params_item_unit_opt = Some(params_item_unit_fn(&mut camel_case_id));
    #[allow(
      // Option will always exist
      clippy::unwrap_used
    )]
    FirParamsItemValues::try_from(ItemWithAttrSpan {
      content: (),
      item: params_item_unit_opt.as_mut().unwrap(),
      span: Span::mixed_site(),
    })?
  };
  let fasiv = fiv.after_sending.map(FirAfterSendingItemValues::try_from).transpose()?;
  let fbsiv = fiv.before_sending.map(FirBeforeSendingItemValues::try_from).transpose()?;
  let faiv = fiv.aux.map(FirAuxItemValues::try_from).transpose()?;
  let fresdiv = FirResDataItemValues::try_from(fiv.res_data)?;
  let sea = SirEndpointAttr::try_from(FirPkgAttr::try_from(&*attr_args)?)?;
  let SirFinalValues { auxs, package, package_impls } = SirFinalValues::try_from((
    &mut camel_case_id,
    fpiv,
    freqdiv,
    fresdiv,
    sea,
    fasiv,
    faiv,
    fbsiv,
  ))?;
  if let Some(content) = item_mod.content.as_mut() {
    content.1.push(syn::Item::Verbatim(quote::quote!(
      /// Auto-generated type representing nothing, which means that the corresponding
      /// request does not expect any additional parameter.
      #params_item_unit_opt

      #(#auxs)*
      #package
      #(#package_impls)*
    )));
  }
  Ok(item_mod.into_token_stream().into())
}

fn params_item_unit_fn(camel_case_id: &mut String) -> Item {
  Item::Type(ItemType {
    attrs: Vec::new(),
    vis: Visibility::Public(VisPublic { pub_token: Pub(Span::mixed_site()) }),
    type_token: Type(Span::mixed_site()),
    ident: {
      let idx = camel_case_id.len();
      camel_case_id.push_str("Params");
      let ident = Ident::new(camel_case_id, proc_macro2::Span::mixed_site());
      camel_case_id.truncate(idx);
      ident
    },
    generics: Generics {
      lt_token: None,
      params: Punctuated::new(),
      gt_token: None,
      where_clause: None,
    },
    eq_token: Eq(Span::mixed_site()),
    ty: Box::new(unit_type()),
    semi_token: Semi(Span::mixed_site()),
  })
}
