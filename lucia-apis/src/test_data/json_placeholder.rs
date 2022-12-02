//! Fake data for testing and prototyping.
//!
//! <http://jsonplaceholder.typicode.com>
//!
//! ```rust,no_run
//! # async fn fun() -> lucia_apis::Result<()> {
//! use lucia::{
//!   dnsn::SerdeJson,
//!   network::{HttpMethod, HttpParams},
//! };
//! use lucia_apis::{
//!   misc::PackagesAux,
//!   test_data::json_placeholder::{GenericParams, JsonPlaceholder},
//! };
//!
//! let mut pkgs_aux =
//!   PackagesAux::from_minimum(JsonPlaceholder, SerdeJson, (HttpParams::from_url("URL")?));
//! let _ = pkgs_aux.albums().params(GenericParams::new(None, HttpMethod::Get, None, &[])).build();
//! # Ok(()) }
//! ```

#[cfg(all(test, feature = "_integration-tests"))]
mod integration_tests;
mod pkg;

use crate::misc::PackagesAux;
use lucia::{network::HttpParams, Api};
pub use pkg::*;

pub(crate) type JsonPlaceholderHttpPackagesAux<DRSR> =
  PackagesAux<JsonPlaceholder, DRSR, HttpParams>;

#[doc = _generic_api_doc!()]
#[derive(Debug)]
pub struct JsonPlaceholder;

impl Api for JsonPlaceholder {
  type Error = crate::Error;
}
