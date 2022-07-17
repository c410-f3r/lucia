//! Ethereum is a decentralized, open-source blockchain with smart contract functionality.
//!
//! Credits goes to <https://github.com/tomusdrw/rust-web3>.
//!
//! ```rust,no_run
//! # async fn fun() -> lucia::Result<()> {
//! use lucia::{
//!   network::{HttpParams, Transport},
//!   Pair,
//! };
//! let (mut rm, mut trans) = Pair::new((), HttpParams::from_origin("ORIGIN")?).into_parts();
//! let req = rm.eth_block_number();
//! let _res = trans.send_retrieve_and_decode_one(&mut rm, &req, ()).await?;
//! Ok(())
//! # };
//! ```
#![cfg(feature = "ethereum")]

mod access_list;
mod access_list_item;
mod block_id;
mod block_number;
mod call_request;
mod contract;
mod endpoint;
mod integration_tests;
mod raw_transaction;
mod receipt;
mod transaction;
mod transaction_condition;
mod transaction_request;
mod types;

pub use access_list::*;
pub use access_list_item::*;
pub use block_id::*;
pub use block_number::*;
pub use call_request::*;
pub use contract::*;
pub use endpoint::*;
pub use raw_transaction::*;
pub use receipt::*;
pub use transaction::*;
pub use transaction_condition::*;
pub use transaction_request::*;
pub use types::*;

#[derive(Debug)]
pub struct Ethereum;
