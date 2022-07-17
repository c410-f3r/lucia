//! Realtime and historical data on Coronavirus COVID-19 confirmed cases, deaths, and recovered cases.
//!
//! ```rust,no_run
//! # async fn fun() -> lucia::Result<()> {
//! use lucia::{
//!   api::health::covid_19::CasesParams,
//!   network::{HttpParams, Transport},
//!   Pair,
//! };
//! let (mut rm, mut trans) = Pair::new((), HttpParams::from_origin("ORIGIN")?).into_parts();
//! let req = rm.cases();
//! let _res = trans.send_retrieve_and_decode_one(
//!   &mut rm,
//!   &req,
//!   CasesParams::new(None, None, None)
//! ).await?;
//! Ok(())
//! # };
//! ```

#![cfg(feature = "covid-19")]

mod endpoint;
//mod integration_tests; Flaky

pub use endpoint::*;

use arrayvec::ArrayString;

#[derive(Debug)]
pub struct Covid19;

#[derive(Debug, serde::Deserialize)]
pub struct CountryInfo {
  pub country: Option<ArrayString<32>>,
  pub population: Option<u64>,
  pub sq_km_area: Option<f32>,
  pub continent: Option<ArrayString<16>>,
  pub abbreviation: Option<ArrayString<2>>,
  pub location: Option<ArrayString<28>>,
  pub iso: Option<u32>,
  pub capital_city: Option<ArrayString<24>>,
}
