//! # Lucia
//!
//! A flexible client API framework as well as a set of API collections.

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]

extern crate alloc;

#[macro_use]
mod macros;

pub mod api;
pub mod consts;
pub mod network;
pub mod protocol;
pub mod types;
pub mod utils;

mod client;
mod error;
mod request;
mod tuple_impls;

pub use api::Api;
pub use client::Client;
pub use error::Error;
pub use request::{Request, RequestBuilder, RequestWithAux, Requests};

/// Alias of `core::result::Result<T, lucia::Error>`
pub type Result<T> = core::result::Result<T, Error>;
