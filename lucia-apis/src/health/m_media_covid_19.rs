//! Realtime and historical data on Coronavirus COVID-19 confirmed cases, deaths, and recovered cases.
//!
//! <https://github.com/M-Media-Group/Covid-19-API>
//!
//! ```rust,no_run
//! # async fn fun() -> lucia_apis::Result<()> {
//! use lucia::{
//!   misc::{CommonParams, Pair},
//!   network::{http::ReqParams, Transport},
//! };
//! use lucia_apis::{
//!   health::m_media_covid_19::CasesParams,
//!   misc::RequestManagerWrapper,
//! };
//!
//! let (mut rm, mut trans) = Pair::new(
//!   RequestManagerWrapper::new(
//!     <_>::default(),
//!     CommonParams::new(ReqParams::from_origin("ORIGIN")?, ()),
//!     ()
//!   ),
//!   ()
//! ).into_parts();
//! let req = rm.cases();
//! let _res = trans.send_and_retrieve(
//!   &mut rm,
//!   &req,
//!   CasesParams::new(None, None, None)
//! ).await?;
//! # Ok(()) }
//! ```

mod endpoint;
#[cfg(all(test, feature = "_integration-tests"))]
mod integration_tests;

pub use endpoint::*;

use arrayvec::ArrayString;

#[derive(Debug, Default)]
pub struct MMediaCovid19;

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Debug)]
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
