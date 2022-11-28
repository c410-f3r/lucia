//! Solana is a public blockchain platform with smart contract functionality.
//!
//! <https://docs.solana.com/developing/clients/jsonrpc-api>
//!
//! ```rust,no_run
//! # async fn fun() -> lucia_apis::Result<()> {
//! use lucia::{dnsn::SerdeJson, network::HttpParams};
//! use lucia_apis::{blockchain::solana::Solana, misc::PackagesAux};
//!
//! let mut pkgs_aux = PackagesAux::from_minimum(Solana, SerdeJson, HttpParams::from_url("URL")?);
//! let _ = pkgs_aux.get_slot().data(None, None).build();
//! # Ok(()) }
//! ```

#[allow(missing_docs)]
mod account;
#[allow(missing_docs)]
mod block;
#[allow(missing_docs)]
mod filter;
#[cfg(all(test, feature = "_integration-tests"))]
mod integration_tests;
#[allow(missing_docs)]
mod notification;
mod pkg;
#[allow(missing_docs)]
pub mod program;
#[allow(missing_docs)]
mod reward;
#[cfg(feature = "serde")]
mod short_vec;
#[allow(missing_docs)]
mod slot_update;
#[allow(missing_docs)]
mod transaction;

pub use account::*;
pub use block::*;
pub use filter::*;
pub use notification::*;
pub use pkg::*;
pub use reward::*;
pub use slot_update::*;
pub use transaction::*;

use crate::{
  blockchain::{
    solana::pkg::{GetSignatureStatusesReqData, GetSignatureStatusesResElem},
    ConfirmTransactionOptions,
  },
  misc::PackagesAux,
};
use arrayvec::ArrayString;
use lucia::{
  data_format::{JsonRpcRequest, JsonRpcResponse},
  network::{transport::Transport, HttpParams, WsParams},
  package::Package,
};

pub(crate) const MAX_TRANSACTION_ACCOUNTS_NUM: usize = 240;

pub(crate) type Epoch = u64;
pub(crate) type SolanaHttpPackagesAux<DRSR> = PackagesAux<Solana, DRSR, HttpParams>;
pub(crate) type SolanaProgramName = ArrayString<32>;
pub(crate) type SolanaWsPackagesAux<DRSR> = PackagesAux<Solana, DRSR, WsParams>;

_create_blockchain_constants!(
  pub address_hash: SolanaAddressHash = 32,
  pub address_hash_str: SolanaAddressHashStr = 44,
  pub block_hash: SolanaBlockhash = 32,
  pub block_hash_str: SolanaBlockhashStr = 44,
  pub signature_hash: SolanaSignatureHash = 64,
  pub signature_hash_str: SolanaSignatureHashStr = 90,
  pub transaction_hash: SolanaTransactionHash = 64,
  pub transaction_hash_str: SolanaTransactionHashStr = 90
);

#[derive(Debug)]
#[doc = _generic_dummy_api_doc!()]
pub struct Solana;

impl Solana {
  /// Make successive HTTP requests over a period defined in `cto` until the transaction is
  /// successful or expired.
  #[inline]
  pub async fn confirm_transaction<'tx_hash, DRSR, T>(
    cto: ConfirmTransactionOptions,
    pkgs_aux: &mut SolanaHttpPackagesAux<DRSR>,
    trans: &mut T,
    tx_hash: &'tx_hash str,
  ) -> crate::Result<bool>
  where
    DRSR: Send + Sync,
    T: Transport<DRSR, Params = HttpParams> + Send + Sync,
    for<'signs> GetSignatureStatusesPkg<JsonRpcRequest<GetSignatureStatusesReqData<'signs, &'tx_hash str>>>:
      Package<
        DRSR,
        T::Params,
        Api = Solana,
        Error = crate::Error,
        ExternalResponseContent = JsonRpcResponse<GetSignatureStatusesResData>,
      >,
  {
    macro_rules! call {
      () => {{
        let signatures = &[tx_hash];
        let pkg = &mut pkgs_aux.get_signature_statuses().data(signatures, None).build();
        if let Some(Some(GetSignatureStatusesResElem {
          confirmation_status: Commitment::Finalized,
          ..
        })) = trans.send_retrieve_and_decode_contained(pkg, pkgs_aux).await?.result?.value.get(0)
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
