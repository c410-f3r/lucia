use crate::{
  api::blockchain::{
    solana::{
      endpoint::{Commitment, GetSignatureStatusesRes},
      program::spl_token,
      AccountData, AccountDataJson, AccountDataJsonParsed, Solana,
    },
    ConfirmTransactionOptions,
  },
  network::Transport,
  RequestBuilder,
};
use core::time::Duration;

const DEFAULT_CTO: ConfirmTransactionOptions = ConfirmTransactionOptions::TriesWithInterval {
  interval: Duration::from_millis(1000),
  number: 60 * 3,
};

impl Solana {
  #[inline]
  pub async fn confirm_transaction<T>(
    cto_opt: Option<ConfirmTransactionOptions>,
    id: &str,
    rb: &mut RequestBuilder<Self>,
    trans: &mut T,
  ) -> crate::Result<bool>
  where
    T: Send + Transport,
  {
    macro_rules! call {
      () => {{
        let req = rb.get_signature_statuses(None, [id].into());
        if let &[Some(GetSignatureStatusesRes {
          confirmation_status: Commitment::Finalized, ..
        }), ..] = &*trans.send_retrieve_and_decode_one(&req, rb.tp_mut()).await?.result.value
        {
          true
        } else {
          false
        }
      }};
    }

    match cto_opt.unwrap_or(DEFAULT_CTO) {
      ConfirmTransactionOptions::Tries { number } => {
        for _ in 0u32..number {
          if call!() {
            return Ok(true);
          }
        }
      }
      ConfirmTransactionOptions::TriesWithInterval { interval, number } => {
        for _ in 0u32..number {
          if call!() {
            return Ok(true);
          }
          crate::utils::_sleep(interval).await;
        }
      }
    }

    Ok(false)
  }

  // If existing, extracts the parsed spl token account information of a generic [AccountData].
  #[inline]
  pub fn spl_token_account(account_data: &AccountData) -> crate::Result<&spl_token::Account> {
    if let &AccountData::Json(AccountDataJson {
      parsed:
        AccountDataJsonParsed::SplTokenAccount(spl_token::GenericAccount::Account(ref account)),
      ..
    }) = account_data
    {
      Ok(account)
    } else {
      Err(crate::Error::SolanaAccountIsNotSplToken)
    }
  }
}
