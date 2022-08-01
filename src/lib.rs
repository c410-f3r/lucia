#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

#[macro_use]
mod macros;

pub mod api;
pub mod data_format;
pub mod dnsn;
pub mod network;
pub mod types;
pub mod utils;

mod common_params;
mod consts;
mod error;
mod pair;
mod req_res;

pub use common_params::*;
pub use error::*;
pub use pair::*;
pub use req_res::*;

/// Alias of `core::result::Result<T, lucia::Error>`
pub type Result<T> = core::result::Result<T, Error>;
