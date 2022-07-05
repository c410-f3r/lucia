//! Many parts of the Ethereum code were based on <https://github.com/tomusdrw/rust-web3>.

pub mod contract;
pub mod endpoint;

mod access_list;
mod access_list_item;
mod block_id;
mod block_number;
mod call_request;
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
pub use raw_transaction::*;
pub use receipt::*;
pub use transaction::*;
pub use transaction_condition::*;
pub use transaction_request::*;
pub use types::*;

use crate::{api::Api, types::MaxUrl, utils::RequestThrottling};

#[derive(Debug)]
pub struct Ethereum {
  origin: MaxUrl,
  rt: Option<RequestThrottling>,
}

impl Api for Ethereum {
  type Aux = Option<RequestThrottling>;

  #[inline]
  fn new(origin: &str, rt: Self::Aux) -> crate::Result<Self> {
    Ok(Self { origin: origin.try_into()?, rt })
  }

  #[inline]
  fn origin(&self) -> &MaxUrl {
    &self.origin
  }
}
