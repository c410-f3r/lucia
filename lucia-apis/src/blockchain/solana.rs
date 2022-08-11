//! Solana is a public blockchain platform with smart contract functionality.
//!
//! <https://docs.solana.com/developing/clients/jsonrpc-api>
//!
//! ```rust,no_run
//! # async fn fun() -> lucia_apis::Result<()> {
//! use lucia::{
//!   misc::{CommonParams, Pair},
//!   network::{http::ReqParams, Transport},
//! };
//! use lucia_apis::misc::RequestManagerWrapper;
//!
//! let (mut rm, mut trans) = Pair::new(
//!   RequestManagerWrapper::new(
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

use crate::{blockchain::ConfirmTransactionOptions, misc::RequestManagerWrapper};
use arrayvec::{ArrayString, ArrayVec};
use lucia::{
  data_formats::{JsonRpcRequest, JsonRpcResponse, ProcessedJsonRpcResponse},
  dnsn::Serialize,
  network::Transport,
  req_res::Request,
};

pub(crate) const MAX_BINARY_DATA_LEN: usize = 1024;
pub(crate) const MAX_TRANSACTION_ACCOUNTS_NUM: usize = 240;

pub(crate) type Epoch = u64;
pub(crate) type SolanaLogMessage = ArrayString<96>;
pub(crate) type SolanaProgramName = ArrayString<32>;

_create_blockchain_constants!(
  pub address_hash: SolanaAddressHash = 32,
  pub address_hash_str: SolanaAddressHashStr = 44,
  pub block_hash: SolanaBlockhash = 32,
  pub block_hash_str: SolanaBlockhashStr = 44,
  pub signature_hash: SolanaSignatureHash = 64,
  pub signature_hash_str: SolanaSignatureHashStr = 90,
  pub transaction_hash: _SolanaTransactionHash = 64,
  pub transaction_hash_str: SolanaTransactionHashStr = 90
);

#[derive(Debug, Default)]
pub struct Solana;

impl Solana {
  #[inline]
  pub async fn confirm_transaction<'tx_hash, CP, DRSR, T>(
    cto: ConfirmTransactionOptions,
    tx_hash: &'tx_hash str,
    rm: &mut RequestManagerWrapper<Self, CP, DRSR>,
    trans: &mut T,
  ) -> crate::Result<bool>
  where
    CP: Send,
    DRSR: Send,
    T: Send + Transport<Self, CP, DRSR>,
    JsonRpcRequest<GetSignatureStatusesReq<1, &'tx_hash str>>: Request<
        CP,
        DRSR,
        (),
        T::ResponseParams,
        Error = crate::Error,
        ProcessedResponse = ProcessedJsonRpcResponse<
          JsonRpcResponseResultWithContext<ArrayVec<Option<GetSignatureStatusesRes>, 1>>,
        >,
        RawResponse = JsonRpcResponse<
          JsonRpcResponseResultWithContext<ArrayVec<Option<GetSignatureStatusesRes>, 1>>,
        >,
      > + Serialize<DRSR>,
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

    match cto {
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
          lucia::misc::sleep(interval).await?;
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
