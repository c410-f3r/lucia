//! Ethereum is a decentralized, open-source blockchain with smart contract functionality.
//!
//! <https://web3js.readthedocs.io/>
//!
//! Built upon the logic constructed in <https://github.com/tomusdrw/rust-web3>.
//!
//! ```rust,no_run
//! # async fn fun() -> lucia_apis::Result<()> {
//! use lucia::{dnsn::SerdeJson, network::HttpParams};
//! use lucia_apis::{blockchain::ethereum::Ethereum, misc::PkgsAux};
//!
//! let mut pkgs_aux = PkgsAux::from_minimum(Ethereum, SerdeJson, HttpParams::from_url("URL")?);
//! let _ = pkgs_aux.eth_block_number().build();
//! # Ok(()) }
//! ```

mod access_list;
mod access_list_item;
mod block_id;
mod block_number;
mod call_request;
//mod contract;
#[cfg(all(test, feature = "_integration-tests"))]
mod integration_tests;
mod pkg;
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
//pub use contract::Contract;
use lucia::Api;
pub use pkg::*;
pub use raw_transaction::*;
pub use receipt::*;
pub use transaction::*;
pub use transaction_condition::*;
pub use transaction_request::*;
pub use types::*;

#[derive(Debug)]
#[doc = _generic_api_doc!()]
#[lucia_macros::api_types(pkgs_aux(crate::misc::PkgsAux), transport(http, ws))]
pub struct Ethereum;

#[cfg_attr(feature = "async-trait", async_trait::async_trait)]
impl Api for Ethereum {
  type Error = crate::Error;

  async fn before_sending(&mut self) -> Result<(), Self::Error> {
    Ok(())
  }
}
