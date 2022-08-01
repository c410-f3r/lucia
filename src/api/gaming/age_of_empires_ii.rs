//! Simple API to retrieve resources related to Age of Empires II.
//!
//! <https://age-of-empires-2-api.herokuapp.com/docs>
//!
//! ```rust,no_run
//! # async fn fun() -> lucia::Result<()> {
//! use lucia::{
//!   api::gaming::age_of_empires_ii::UnitsParams,
//!   network::{http::ReqParams, Transport},
//!   CommonParams, Pair, RequestManager,
//! };
//! let (mut rm, mut trans) = Pair::new(
//!   RequestManager::new(
//!     <_>::default(),
//!     CommonParams::new(ReqParams::from_origin("ORIGIN")?, ()),
//!     ()
//!   ),
//!   ()
//! ).into_parts();
//! let req = rm.units();
//! let _res = trans.send_and_retrieve(&mut rm, &req, UnitsParams::new()).await?;
//! # Ok(()) }
//! ```

mod endpoint;
#[cfg(all(test, feature = "_integration-tests"))]
mod integration_tests;

pub use endpoint::*;

#[derive(Debug, Default)]
pub struct AgeOfEmpiresII;

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
#[derive(Debug)]
pub struct CostRes {
  pub wood: Option<u16>,
  pub food: Option<u16>,
  pub stone: Option<u16>,
  pub gold: Option<u16>,
}
