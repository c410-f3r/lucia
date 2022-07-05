//! # Lucia
//!
//! A flexible client API framework as well as a set of API collections.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

#[macro_use]
mod macros;

pub mod api;
pub mod network;
pub mod protocol;
pub mod types;
pub mod utils;

mod consts;
mod error;
mod pair;
mod request;
mod tuple_impls;

pub use error::Error;
pub use pair::Pair;
pub use request::{Request, RequestBuilder, RequestWithAux, Requests};

/// Alias of `core::result::Result<T, lucia::Error>`
pub type Result<T> = core::result::Result<T, Error>;
