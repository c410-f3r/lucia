#![doc = include_str!("../../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

#[macro_use]
mod macros;

pub mod data_formats;
pub mod dnsn;
pub mod misc;
pub mod network;
pub mod req_res;

mod error;

pub use error::*;

/// The maximum length for both requests and responses
pub(crate) const MAX_JSON_RPC_METHOD_LEN: usize = 64;

/// Alias of `core::result::Result<T, lucia::Error>`
pub type Result<T> = core::result::Result<T, Error>;
/// Identifier used to track the number of issued requests.
pub type Id = usize;
