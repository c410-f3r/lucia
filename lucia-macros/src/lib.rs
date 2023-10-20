//! Lucia - Macros

#![allow(
  // Unimportant
  clippy::too_many_lines,
  // Syn stuff
  clippy::wildcard_in_or_patterns
)]

mod api_types;
mod contained_attrs;
mod error;
mod item_with_attr_span;
mod misc;
mod owned_or_ref;
mod pkg;
mod transport_group;

use error::Error;

type Result<T> = core::result::Result<T, Error>;

/// API types
///
/// Creates types referring an API and its possible de-serializers/serializers and transport
/// variants.
#[proc_macro_attribute]
pub fn api_types(
  attrs: proc_macro::TokenStream,
  item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
  match api_types::api_types(attrs, item) {
    Err(err) => syn::Error::from(err).to_compile_error().into(),
    Ok(elem) => elem,
  }
}

/// Package
///
/// A framework-like attribute placed in inline modules that creates all the mandatory elements
/// and optional elements related to `lucia::pkg::Package`.
///
/// ```rust
/// struct SomeApi;
///
/// #[lucia_macros::pkg(api(SomeApi), data_format(json_rpc("SomeEndpoint")))]
/// mod pkg {
///   #[pkg::req_data]
///   pub struct SomeEndpointReq<'string> {
///     ping: &'string str,
///   }
///
///   #[pkg::res_data]
///   pub struct SomeEndpointRes {
///     pong: String,
///   }
/// }
/// ```
#[proc_macro_attribute]
pub fn pkg(
  attr: proc_macro::TokenStream,
  item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
  match pkg::pkg(attr, item) {
    Err(err) => syn::Error::from(err).to_compile_error().into(),
    Ok(elem) => elem,
  }
}
