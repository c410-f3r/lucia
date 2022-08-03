//! Auxiliary network function and structures as well as different transport implementations

pub mod http;
mod transport;
pub mod ws;

pub use transport::*;
