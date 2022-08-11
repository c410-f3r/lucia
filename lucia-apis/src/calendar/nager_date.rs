//! Public holidays for more than 100 countries.
//!
//! <https://date.nager.at>
//!
//! ```rust,no_run
//! # async fn fun() -> lucia_apis::Result<()> {
//! use lucia::{
//!   network::{http::ReqParams, Transport},
//!   misc::{CommonParams, Pair}
//! };
//! use lucia_apis::{
//!   calendar::nager_date::V3CountryInfoParams,
//!   misc::RequestManagerWrapper
//! };
//! let (mut rm, mut trans) = Pair::new(
//!   RequestManagerWrapper::new(
//!     <_>::default(),
//!     CommonParams::new(ReqParams::from_origin("ORIGIN")?, ()),
//!     ()
//!   ),
//!   ()
//! ).into_parts();
//! let req = rm.v3_country_info();
//! let _res = trans.send_and_retrieve(&mut rm, &req, V3CountryInfoParams::new("es")).await?;
//! # Ok(()) }
//! ```

mod endpoint;
#[cfg(all(test, feature = "_integration-tests"))]
mod integration_tests;

pub use endpoint::*;

#[derive(Debug, Default)]
pub struct NagerDate;
