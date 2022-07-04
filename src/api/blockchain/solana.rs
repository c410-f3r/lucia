mod account;
pub mod endpoint;
mod filter;
#[cfg(all(test, feature = "_integration-tests"))]
mod integration_tests;
mod notification;
pub mod program;
mod short_vec;
mod solana_impl;
mod transaction;
mod utils;

pub use account::*;
pub use filter::*;
pub use notification::*;
pub use transaction::*;

use crate::{types::MaxUrl, Api};
use arrayvec::ArrayString;
use utils::*;

pub(crate) const MAX_BINARY_DATA_LEN: usize = 1024;
pub(crate) const MAX_TRANSACTION_ACCOUNTS_NUM: usize = 240;

pub type SolanaClient<T> = crate::Client<Solana, T>;

pub(crate) type Epoch = u64;
pub(crate) type SolanaLogMessage = ArrayString<96>;
pub(crate) type SolanaProgramName = ArrayString<32>;

_create_blockchain_constants!(
  address_hash: SolanaAddressHash = 32,
  block_hash: SolanaBlockhash = 32,
  signature_hash: SolanaSignatureHash = 64,
  transaction_hash: SolanaTransactionHash = 64,
  //
  address_hash_str: SolanaAddressHashStr = 44,
  block_hash_str: SolanaBlockhashStr = 44,
  signature_hash_str: SolanaSignatureHashStr = 90,
  transaction_hash_str: SolanaTransactionHashStr = 90
);

#[derive(Debug)]
pub struct Solana {
  origin: MaxUrl,
}

impl Api for Solana {
  #[inline]
  fn from_origin(origin: &str) -> crate::Result<Self> {
    Ok(Self { origin: origin.try_into()? })
  }

  #[inline]
  fn origin(&self) -> &MaxUrl {
    &self.origin
  }
}
