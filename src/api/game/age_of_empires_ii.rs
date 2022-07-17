//! Simple API to retrieve resources related to Age of Empires II.
//!
//! ```rust,no_run
//! # async fn fun() -> lucia::Result<()> {
//! use lucia::{
//!   api::game::age_of_empires_ii::UnitsParams,
//!   network::{HttpParams, Transport},
//!   Pair,
//! };
//! let (mut rm, mut trans) = Pair::new((), HttpParams::from_origin("ORIGIN")?).into_parts();
//! let req = rm.units();
//! let _res = trans.send_retrieve_and_decode_one(&mut rm, &req, UnitsParams::new()).await?;
//! Ok(())
//! # };
//! ```

#![cfg(feature = "age-of-empires-ii")]

mod endpoint;
mod integration_tests;

pub use endpoint::*;

#[derive(Debug)]
pub struct AgeOfEmpiresII;
