//! Public holidays for more than 100 countries.
//!
//! <https://date.nager.at>
//!
//! ```rust,no_run
//! # async fn fun() -> lucia_apis::Result<()> {
//! use lucia::{dnsn::SerdeJson, network::HttpParams};
//! use lucia_apis::{calendar::nager_date::NagerDate, misc::PkgsAux};
//!
//! let mut pkgs_aux = PkgsAux::from_minimum(NagerDate, SerdeJson, HttpParams::from_url("URL")?);
//! let _ = pkgs_aux.v3_country_info().params("es").build();
//! # Ok(()) }
//! ```

#[cfg(all(test, feature = "_integration-tests"))]
mod integration_tests;
mod pkg;

use lucia::Api;
pub use pkg::*;

#[derive(Debug)]
#[doc = _generic_api_doc!()]
#[lucia_macros::api_types(pkgs_aux(crate::misc::PkgsAux), transport(http))]
pub struct NagerDate;

impl Api for NagerDate {
  type Error = crate::Error;
}
