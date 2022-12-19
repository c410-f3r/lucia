//! Collection of APIs based on the lucia framework.
//!
//! Most of the API structures are markers used to guide different type implementations.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "async-trait"), allow(incomplete_features))]
#![cfg_attr(not(feature = "async-trait"), feature(async_fn_in_trait, impl_trait_projections))]

extern crate alloc;

#[macro_use]
mod macros;

pub mod blockchain;
pub mod calendar;
mod error;
pub mod exchange;
pub mod misc;
pub mod series;
pub mod test_data;

pub use error::Error;
/// Alias of `core::result::Result<T, lucia_apis::Error>`
pub type Result<T> = core::result::Result<T, Error>;
