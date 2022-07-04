use crate::{
  api::blockchain::{
    solana::{
      endpoint::{Commitment, GetSignatureStatusesRes, SendTransactionParameters},
      program::spl_token,
      AccountData, AccountDataJson, AccountDataJsonParsed, Solana, SolanaAddressHash, SolanaClient,
      SolanaTransactionHashStr, TransactionInput,
    },
    ConfirmTransactionOptions,
  },
  network::Transport,
  RequestBuilder,
};
use alloc::vec::Vec;
use core::{future::Future, time::Duration};

const DEFAULT_CTO: ConfirmTransactionOptions = ConfirmTransactionOptions::TriesWithInterval {
  interval: Duration::from_millis(1000),
  number: 60 * 3,
};

impl Solana {
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
      #[cfg(any(feature = "tokio", feature = "async-std"))]
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

  #[inline]
  pub async fn send_transaction<T>(
    buffer: &mut Vec<u8>,
    opt: Option<SendTransactionParameters>,
    rb: &mut RequestBuilder<Self>,
    transaction: &TransactionInput,
    trans: &mut T,
  ) -> crate::Result<SolanaTransactionHashStr>
  where
    T: Send + Transport,
  {
    let req = rb.send_transaction(buffer, opt, transaction)?;
    Ok(trans.send_retrieve_and_decode_one(&req, rb.tp_mut()).await?.result)
  }

  #[inline]
  pub async fn try_with_blockhashes<A, E, F1, F2, R, T>(
    additional_tries: u8,
    aux: A,
    initial_blockhash: SolanaAddressHash,
    mut client_cb: impl FnMut(A) -> F1,
    mut cb: impl FnMut(A, SolanaClient<T>, SolanaAddressHash) -> F2,
  ) -> Result<(R, Option<SolanaAddressHash>), E>
  where
    A: Copy,
    E: From<crate::Error>,
    F1: Future<Output = SolanaClient<T>>,
    F2: Future<Output = Result<R, E>>,
    T: Send + Transport,
  {
    macro_rules! call {
      () => {{
        let mut client = client_cb(aux).await;
        let (rb, trans) = client.parts_mut();
        let req = rb.get_latest_blockhash(None);
        let res = trans.send_retrieve_and_decode_one(&req, rb.tp_mut()).await?;
        let blockhash = res.result.value.blockhash;
        cb(aux, client, blockhash).await.map(|rslt| (rslt, blockhash))
      }};
    }
    let initial_client = client_cb(aux).await;
    match cb(aux, initial_client, initial_blockhash).await {
      Err(err) => {
        if let Some(n) = additional_tries.checked_sub(1) {
          for _ in 0..n {
            if let Ok(elem) = call!() {
              return Ok((elem.0, Some(elem.1)));
            }
          }
          let last = call!()?;
          Ok((last.0, Some(last.1)))
        } else {
          Err(err)
        }
      }
      Ok(elem) => Ok((elem, None)),
    }
  }
}
