//! Collection of APIs based on the lucia framework.
//!
//! Most of the API structures are markers used to guide different type implementations.

#![allow(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

#[macro_use]
mod macros;

pub mod art_and_design;
pub mod blockchain;
pub mod calendar;
pub mod exchange;
pub mod gaming;
pub mod health;
pub mod misc;
pub mod test_data;

mod error;

pub use error::Error;

/// Alias of `core::result::Result<T, lucia_apis::Error>`
pub type Result<T> = core::result::Result<T, Error>;
