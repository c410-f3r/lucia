//! Fake data for testing and prototyping.
//!
//! <http://jsonplaceholder.typicode.com>
//!
//! ```rust,no_run
//! # async fn fun() -> lucia_apis::Result<()> {
//! use lucia::{
//!   misc::{CommonParams, Pair},
//!   network::{http::{Method, ReqParams}, Transport},
//! };
//! use lucia_apis::{
//!   misc::RequestManagerWrapper,
//!   test_data::json_placeholder::AlbumsParams
//! };
//!
//! let (mut rmw, mut trans) = Pair::new(
//!   RequestManagerWrapper::new(
//!     <_>::default(),
//!     CommonParams::new(ReqParams::from_origin("ORIGIN")?, ()),
//!     ()
//!   ),
//!   ()
//! ).into_parts();
//! let req = rmw.albums();
//! let _res = trans.send_and_retrieve(
//!   &mut rmw,
//!   &req,
//!   AlbumsParams::new(Method::Get, None, None, &[])
//! ).await?;
//! # Ok(()) }
//! ```

mod endpoint;
#[cfg(all(test, feature = "_integration-tests", feature = "serde"))]
mod integration_tests;

pub use endpoint::*;

#[derive(Debug, Default)]
pub struct JsonPlaceholder;
