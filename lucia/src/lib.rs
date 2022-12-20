#![allow(incomplete_features)]
#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]
#![feature(async_fn_in_trait, impl_trait_projections)]

extern crate alloc;

#[macro_use]
mod macros;

mod api;
pub mod data_format;
pub mod dnsn;
mod error;
pub mod misc;
pub mod network;
pub mod pkg;

pub use api::Api;
pub use error::Error;
#[cfg(feature = "macros")]
pub use lucia_macros::*;

/// Identifier used to track the number of issued requests.
pub type Id = usize;
/// Alias of `core::result::Result<T, lucia::Error>`
pub type Result<T> = core::result::Result<T, Error>;
