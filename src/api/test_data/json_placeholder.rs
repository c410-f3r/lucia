//! Fake data for testing and prototyping.
//!
//! <http://jsonplaceholder.typicode.com>
//!
//! ```rust,no_run
//! # async fn fun() -> lucia::Result<()> {
//! use lucia::{
//!   api::test_data::json_placeholder::AlbumsParams,
//!   network::{http::{Method, ReqParams}, Transport},
//!   CommonParams, Pair, RequestManager
//! };
//! let (mut rm, mut trans) = Pair::new(
//!   RequestManager::new(
//!     <_>::default(),
//!     CommonParams::new(ReqParams::from_origin("ORIGIN")?, ()),
//!     ()
//!   ),
//!   ()
//! ).into_parts();
//! let req = rm.albums();
//! let _res = trans.send_and_retrieve(
//!   &mut rm,
//!   &req,
//!   AlbumsParams::new(Method::Get, None, None, &[])
//! ).await?;
//! # Ok(()) }
//! ```

mod endpoint;
#[cfg(all(test, feature = "_integration-tests"))]
mod integration_tests;

pub use endpoint::*;

#[derive(Debug, Default)]
pub struct JsonPlaceholder;
