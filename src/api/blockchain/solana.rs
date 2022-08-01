//! Solana is a public blockchain platform with smart contract functionality.
//!
//! <https://docs.solana.com/developing/clients/jsonrpc-api>
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
//! let req = rm.get_slot(None);
//! let _res = trans.send_and_retrieve(&mut rm, &req, ()).await?;
//! # Ok(()) }
//! ```

mod account;
mod endpoint;
mod filter;
#[cfg(all(test, feature = "_integration-tests"))]
mod integration_tests;
mod notification;
pub mod program;
#[cfg(feature = "serde")]
mod short_vec;
mod transaction;

pub use account::*;
pub use endpoint::*;
pub use filter::*;
pub use notification::*;
pub use transaction::*;

use crate::{
  api::blockchain::ConfirmTransactionOptions,
  data_format::{JsonRpcRequest, JsonRpcResponse, ProcessedJsonRpcResponse},
  dnsn::{Deserialize, Serialize},
  network::Transport,
  Request, RequestManager,
};
use arrayvec::{ArrayString, ArrayVec};
use core::time::Duration;

pub(crate) const MAX_BINARY_DATA_LEN: usize = 1024;
pub(crate) const MAX_TRANSACTION_ACCOUNTS_NUM: usize = 240;

const DEFAULT_CTO: ConfirmTransactionOptions = ConfirmTransactionOptions::TriesWithInterval {
  interval: Duration::from_millis(5000),
  number: 60,
};

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

#[derive(Debug, Default)]
pub struct Solana;

impl Solana {
  #[inline]
  pub async fn confirm_transaction<'tx_hash, CP, DRSR, T>(
    cto_opt: Option<ConfirmTransactionOptions>,
    tx_hash: &'tx_hash str,
    rm: &mut RequestManager<Self, CP, DRSR>,
    trans: &mut T,
  ) -> crate::Result<bool>
  where
    CP: Send,
    DRSR: Send,
    T: Send + Transport<Self, CP, DRSR>,
    JsonRpcRequest<GetSignatureStatusesReq<1, &'tx_hash str>>: Request<
        CP,
        (),
        T::Metadata,
        ProcessedResponse = ProcessedJsonRpcResponse<
          JsonRpcResponseResultWithContext<ArrayVec<Option<GetSignatureStatusesRes>, 1>>,
        >,
        RawResponse = JsonRpcResponse<
          JsonRpcResponseResultWithContext<ArrayVec<Option<GetSignatureStatusesRes>, 1>>,
        >,
      > + Serialize<DRSR>,
    JsonRpcResponse<JsonRpcResponseResultWithContext<ArrayVec<Option<GetSignatureStatusesRes>, 1>>>:
      for<'de> Deserialize<'de, DRSR>,
  {
    macro_rules! call {
      () => {{
        let req = rm.get_signature_statuses(None, [tx_hash].into());
        if let &[Some(GetSignatureStatusesRes {
          confirmation_status: Commitment::Finalized, ..
        }), ..] = &*trans.send_retrieve_and_decode_one(rm, &req, ()).await?.result.value
        {
          true
        } else {
          false
        }
      }};
    }

    match cto_opt.unwrap_or(DEFAULT_CTO) {
      ConfirmTransactionOptions::Tries { number } => {
        for _ in 0u16..number {
          if call!() {
            return Ok(true);
          }
        }
      }
      ConfirmTransactionOptions::TriesWithInterval { interval, number } => {
        for _ in 0u16..number {
          if call!() {
            return Ok(true);
          }
          crate::utils::_sleep(interval).await?;
        }
      }
    }

    Ok(false)
  }

  /// If existing, extracts the parsed spl token account ([program::spl_token::MintAccount]) out of
  /// a generic [AccountData].
  #[inline]
  pub fn spl_token_mint_account(
    account_data: &AccountData,
  ) -> crate::Result<&program::spl_token::MintAccount> {
    if let Some(program::spl_token::GenericAccount::Mint(ref elem)) =
      Self::spl_token_account(account_data)
    {
      Ok(elem)
    } else {
      Err(crate::Error::SolanaAccountIsNotSplTokenMint)
    }
  }

  /// If existing, extracts the parsed spl token account ([program::spl_token::Account]) out of
  /// a generic [AccountData].
  #[inline]
  pub fn spl_token_normal_account(
    account_data: &AccountData,
  ) -> crate::Result<&program::spl_token::Account> {
    if let Some(program::spl_token::GenericAccount::Account(ref elem)) =
      Self::spl_token_account(account_data)
    {
      Ok(elem)
    } else {
      Err(crate::Error::SolanaAccountIsNotSplToken)
    }
  }

  #[inline]
  fn spl_token_account(account_data: &AccountData) -> Option<&program::spl_token::GenericAccount> {
    if let &AccountData::Json(AccountDataJson {
      parsed: AccountDataJsonParsed::SplTokenAccount(ref spl_token_account),
      ..
    }) = account_data
    {
      Some(spl_token_account)
    } else {
      None
    }
  }
}
