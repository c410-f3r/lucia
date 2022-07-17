//! Public holidays for more than 100 countries.
//!
//! ```rust,no_run
//! # async fn fun() -> lucia::Result<()> {
//! use lucia::{
//!   api::calendar::nager_date::V3CountryInfoParams,
//!   network::{HttpParams, Transport},
//!   Pair,
//! };
//! let (mut rm, mut trans) = Pair::new((), HttpParams::from_origin("ORIGIN")?).into_parts();
//! let req = rm.v3_country_info();
//! let _res = trans.send_retrieve_and_decode_one(&mut rm, &req, V3CountryInfoParams::new("es")).await?;
//! Ok(())
//! # };
//! ```
#![cfg(feature = "nager-date")]

mod endpoint;
mod integration_tests;

pub use endpoint::*;

#[derive(Debug)]
pub struct NagerDate;
