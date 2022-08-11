//! COLOURlovers is a creative community where people from around the world create and share
//! colors, palettes, patterns, discuss the latest trends and explore colorful articles.
//!
//! The ColourLovers.com API is provided under the Creative Commons Attribution-Noncommercial-Share
//! Alike license. Please refer to <http://www.colourlovers.com/api> for more information on the
//! license and Terms Of Use.
//!
//! ```rust,no_run
//! # async fn fun() -> lucia_apis::Result<()> {
//! use lucia::{
//!   misc::{CommonParams, Pair},
//!   network::{http::ReqParams, Transport},
//! };
//! use lucia_apis::{
//!   art_and_design::colour_lovers::{ColourLovers, StatsParams, StatsTy},
//!   misc::RequestManagerWrapper
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
//! let req = rm.stats();
//! let _res = trans.send_and_retrieve(
//!   &mut rm,
//!   &req,
//!   StatsParams::new(StatsTy::Colors)
//! ).await?;
//! # Ok(()) }
//! ```
mod endpoint;
#[cfg(all(test, feature = "_integration-tests"))]
mod integration_tests;

pub use endpoint::*;

use arrayvec::{ArrayString, ArrayVec};
use lucia::misc::DebugDisplay;

const DATE_LEN: usize = 20;
const TITLE_LEN: usize = 20;
const URL_LEN: usize = 98;
const USER_NAME_LEN: usize = 18;

#[derive(Debug, Default)]
pub struct ColourLovers;

#[derive(Debug)]
pub enum CommonReqTy {
  All,
  New,
  Random,
  Top,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename = "patterns"))]
#[derive(Debug)]
pub struct Colors {
  pub hex: ArrayVec<ArrayString<7>, 5>,
}

#[derive(Debug)]
pub struct ContentParams<'reqp> {
  pub keyword_exact: Option<bool>,
  pub keywords: Option<&'reqp str>,
  pub lover: Option<&'reqp str>,
}

#[derive(Debug)]
pub struct FilterParams<'reqp> {
  pub num_result: Option<u8>,
  pub order_col: Option<&'reqp dyn DebugDisplay>,
  pub result_offset: Option<u8>,
  pub sort_by: Option<&'reqp str>,
}
