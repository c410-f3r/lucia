//! Ethereum is a decentralized, open-source blockchain with smart contract functionality.
//!
//! <https://web3js.readthedocs.io/en/v1.7.4/web3-eth.html>
//!
//! Credits goes to <https://github.com/tomusdrw/rust-web3>.
//!
//! ```rust,no_run
//! # async fn fun() -> lucia::Result<()> {
//! use lucia::{
//!   network::{http::ReqParams, Transport},
//!   CommonParams, Pair, RequestManager
//! };
//! let (mut rm, mut trans) = Pair::new(
//!   RequestManager::new(
//!     <_>::default(),
//!     CommonParams::new(ReqParams::from_origin("ORIGIN")?, ()),
//!     ()
//!   ),
//!   ()
//! ).into_parts();
//! let req = rm.eth_block_number();
//! let _res = trans.send_and_retrieve(&mut rm, &req, ()).await?;
//! # Ok(()) }
//! ```

mod access_list;
mod access_list_item;
mod block_id;
mod block_number;
mod call_request;
mod contract;
mod endpoint;
#[cfg(all(test, feature = "_integration-tests"))]
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

#[derive(Debug, Default)]
pub struct Ethereum;
