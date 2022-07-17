//! Fake data for testing and prototyping.
//!
//! ```rust,no_run
//! # async fn fun() -> lucia::Result<()> {
//! use lucia::{
//!   api::test_data::json_placeholder::AlbumsParams,
//!   network::{HttpParams, Transport},
//!   Pair,
//! };
//! let (mut rm, mut trans) = Pair::new((), HttpParams::from_origin("ORIGIN")?).into_parts();
//! let req = rm.albums();
//! let _res = trans.send_retrieve_and_decode_one(&mut rm, &req, AlbumsParams::new()).await?;
//! Ok(())
//! # };
//! ```

#![cfg(feature = "json-placeholder")]

mod endpoint;
mod integration_tests;

pub use endpoint::*;

#[derive(Debug)]
pub struct JsonPlaceholder;
