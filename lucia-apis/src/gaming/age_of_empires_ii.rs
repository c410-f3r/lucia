//! Simple API to retrieve resources related to Age of Empires II.
//!
//! <https://age-of-empires-2-api.herokuapp.com/docs>
//!
//! ```rust,no_run
//! # async fn fun() -> lucia_apis::Result<()> {
//! use lucia::{dnsn::SerdeJson, network::HttpParams};
//! use lucia_apis::{gaming::age_of_empires_ii::AgeOfEmpiresII, misc::PackagesAux};
//!
//! let mut pkgs_aux =
//!   PackagesAux::from_minimum(AgeOfEmpiresII, SerdeJson, HttpParams::from_url("URL")?);
//! let _ = pkgs_aux.units().build();
//! # Ok(()) }
//! ```

#[cfg(all(test, feature = "_integration-tests"))]
mod integration_tests;
mod pkg;

use crate::misc::PackagesAux;
use lucia::network::HttpParams;
pub use pkg::*;

pub(crate) type AgeOfEmpiresIIHttpPackagesAux<DRSR> = PackagesAux<AgeOfEmpiresII, DRSR, HttpParams>;

#[derive(Debug)]
#[doc = _generic_dummy_api_doc!()]
pub struct AgeOfEmpiresII;

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
#[derive(Debug)]
#[lucia_macros::pkg_doc]
pub struct CostResData {
  pub wood: Option<u16>,
  pub food: Option<u16>,
  pub stone: Option<u16>,
  pub gold: Option<u16>,
}
