//! Lucia - Macros

extern crate alloc;

mod contained_attrs;
mod error;
mod item_with_attr_span;
mod misc;
mod pkg;

use error::Error;

type Result<T> = core::result::Result<T, Error>;

/// Package
///
/// A framework-like attribute placed in inline modules that creates all the mandatory elements
/// and optional elements related to `lucia::package::Package`.
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
