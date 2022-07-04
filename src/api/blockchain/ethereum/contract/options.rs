use crate::api::blockchain::ethereum::{AccessList, TransactionCondition};
use ethereum_types::{U256, U64};

/// Contract Call/Query Options
#[derive(Default, Debug, Eq, PartialEq)]
pub struct Options {
  /// Access list
  pub access_list: Option<AccessList>,
  /// A condition to satisfy before including transaction.
  pub condition: Option<TransactionCondition>,
  /// Fixed gas limit
  pub gas: Option<U256>,
  /// Fixed gas price
  pub gas_price: Option<U256>,
  /// Max fee per gas
  pub max_fee_per_gas: Option<U256>,
  /// miner bribe
  pub max_priority_fee_per_gas: Option<U256>,
  /// Fixed transaction nonce
  pub nonce: Option<U256>,
  /// Transaction type, Some(1) for AccessList transaction, None for Legacy
  pub transaction_type: Option<U64>,
  /// Value to transfer
  pub value: Option<U256>,
}

impl Options {
  /// Create new default `Options` object with some modifications.
  #[inline]
  pub fn with(func: impl FnOnce(&mut Options)) -> Options {
    let mut options = Options::default();
    func(&mut options);
    options
  }
}
