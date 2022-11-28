//! Public holidays for more than 100 countries.
//!
//! <https://date.nager.at>
//!
//! ```rust,no_run
//! # async fn fun() -> lucia_apis::Result<()> {
//! use lucia::{dnsn::SerdeJson, network::HttpParams};
//! use lucia_apis::{calendar::nager_date::NagerDate, misc::PackagesAux};
//!
//! let mut pkgs_aux = PackagesAux::from_minimum(NagerDate, SerdeJson, HttpParams::from_url("URL")?);
//! let _ = pkgs_aux.v3_country_info().params("es").build();
//! # Ok(()) }
//! ```

#[cfg(all(test, feature = "_integration-tests"))]
mod integration_tests;
mod pkg;

use crate::misc::PackagesAux;
use lucia::network::HttpParams;
pub use pkg::*;

pub(crate) type NagerDateHttpPackagesAux<DRSR> = PackagesAux<NagerDate, DRSR, HttpParams>;

#[derive(Debug)]
#[doc = _generic_dummy_api_doc!()]
pub struct NagerDate;
