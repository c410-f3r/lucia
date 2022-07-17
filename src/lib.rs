#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

#[macro_use]
mod macros;

pub mod api;
pub mod network;
pub mod protocol;
pub mod types;
pub mod utils;

mod common_params;
mod consts;
mod error;
mod pair;
mod request;

pub use common_params::*;
pub use error::*;
pub use pair::*;
pub use request::*;

/// Alias of `core::result::Result<T, lucia::Error>`
pub type Result<T> = core::result::Result<T, Error>;
