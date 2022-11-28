use crate::pkg::keywords;
use syn::{
  parse::{Parse, ParseStream},
  spanned::Spanned,
};

#[derive(Debug)]
pub(crate) enum FirAuxFieldAttr {
  AuxData,
  AuxParams,
}

impl Parse for FirAuxFieldAttr {
  fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
    let _ = input.parse::<syn::Token![#]>()?;
    let content;
    syn::bracketed!(content in input);
    let endpoint = content.parse::<keywords::pkg>()?;
    let _ = content.parse::<syn::Token![::]>()?;
    let lookahead = content.lookahead1();
    Ok(if lookahead.peek(keywords::aux_data) {
      let _ = content.parse::<keywords::aux_data>()?;
      Self::AuxData
    } else if lookahead.peek(keywords::aux_params) {
      let _ = content.parse::<keywords::aux_params>()?;
      Self::AuxParams
    } else {
      return Err(crate::Error::BadField(endpoint.span()).into());
    })
  }
}
