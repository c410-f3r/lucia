//! Growing list of records, called blocks, that are securely linked together using cryptography.

mod confirm_transaction_params;
#[cfg(feature = "ethereum")]
pub mod ethereum;
#[cfg(feature = "solana")]
pub mod solana;

pub use confirm_transaction_params::*;
