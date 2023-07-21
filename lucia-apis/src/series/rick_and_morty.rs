//! GraphQL API based on the television show Rick and Morty.
//!
//! <https://github.com/afuh/rick-and-morty-api>
//!
//! ```rust,no_run
//! # async fn fun() -> lucia_apis::Result<()> {
//! use lucia::{dnsn::SerdeJson, network::HttpParams};
//! use lucia_apis::{misc::PkgsAux, series::rick_and_morty::RickAndMorty};
//!
//! let mut pkgs_aux = PkgsAux::from_minimum(RickAndMorty, SerdeJson, HttpParams::from_url("URL")?);
//! let _ = pkgs_aux.character().data(&mut String::new(), 1)?.build();
//! # Ok(()) }
//! ```
//!
//! #### Noteworthy
//!
//! The architecture here is purposely "REST-ish" to avoid third-party dependencies and quickly
//! exercise the actual format expected by GraphQL requests and responses for `lucia`.
#[cfg(all(test, feature = "_integration-tests"))]
mod integration_tests;
mod pkg;

use crate::misc::PkgsAux;
use lucia::Api;
pub use pkg::*;

#[derive(Debug)]
#[doc = _generic_api_doc!()]
#[lucia_macros::api_types(pkgs_aux(PkgsAux), transport(http))]
pub struct RickAndMorty;

#[cfg_attr(feature = "async-trait", async_trait::async_trait)]
impl Api for RickAndMorty {
  type Error = crate::Error;
}
