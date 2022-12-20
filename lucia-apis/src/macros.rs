/// Utility for generic tests
#[macro_export]
macro_rules! create_generic_test {
  ($executor:ident, $test:ident, $pair:expr, $parts_cb:expr, $rslt_cb:expr) => {
    #[$executor::test]
    async fn $test() {
      fn parts_cb_infer<'pair, API, DRSR, O, T>(
        pkgs_aux: &'pair mut $crate::misc::PkgsAux<API, DRSR, T::Params>,
        trans: &'pair mut T,
        cb: impl FnOnce(
          &'pair mut $crate::misc::PkgsAux<API, DRSR, T::Params>,
          &'pair mut T
        ) -> O,
      ) -> O
      where
        T: Transport<DRSR>
      {
        cb(pkgs_aux, trans)
      }
      fn rslt_cb_infer<'pair, API, DRSR, O, R, T>(
        pkgs_aux: &'pair mut $crate::misc::PkgsAux<API, DRSR, T::Params>,
        trans: &'pair mut T,
        rslt: R,
        cb: impl FnOnce(
          &'pair mut $crate::misc::PkgsAux<API, DRSR, T::Params>,
          &'pair mut T,
          R
        ) -> O,
      ) -> O
      where
      T: Transport<DRSR>
      {
        cb(pkgs_aux, trans, rslt)
      }
      init_test_cfg();
      let mut pair = $pair;
      let (pkg, pkgs_aux) = pair.parts_mut();
      let rslt = parts_cb_infer(pkg, pkgs_aux, $parts_cb).await;
      rslt_cb_infer(pkg, pkgs_aux, rslt, $rslt_cb).await;
    }
  };
}

/// Utility for HTTP tests
#[macro_export]
macro_rules! create_http_test {
  ($api:expr, $drsr_erp:expr, $test:ident, $cb:expr) => {
    $crate::create_generic_test! {
      tokio,
      $test,
      {
        let (drsr, ext_req_params) = $drsr_erp;
        lucia::misc::Pair::new(
          $crate::misc::PkgsAux::from_minimum($api, drsr, ext_req_params),
          reqwest::Client::default()
        )
      },
      $cb,
      |_, _, _| async {}
    }
  };
}

/// Utility for WebSocket tests
#[macro_export]
macro_rules! create_ws_test {
  (
    $url:expr,
    $api:expr,
    $drsr_erp:expr,
    $sub:ident,
    ($($unsub:ident),+),
    $cb:expr
  ) => {
    $crate::create_generic_test! {
      tokio,
      $sub,
      {
        let (drsr, ext_req_params) = $drsr_erp;
        let (trans, _) = tokio_tungstenite::connect_async($url).await.unwrap();
        lucia::misc::Pair::new(
          $crate::misc::PkgsAux::from_minimum($api, drsr, ext_req_params),
          trans
        )
      },
      $cb,
      |pkgs_aux, trans, subs| async move {
        let mut iter = subs.into_iter();
        let ids = &mut [$( pkgs_aux.$unsub().data(iter.next().unwrap()).build(), )+][..];
        let _res = trans.send(&mut lucia::pkg::BatchPkg::new(ids), pkgs_aux).await.unwrap();
      }
    }
  };
}

/// Sometimes a received blockhash is not valid so this macro tries to perform additional calls
/// with different blockhashes.
#[macro_export]
#[cfg(feature = "solana")]
macro_rules! try_with_solana_blockhashes {
  (
    let $local_blockhash:ident = $initial_blockhash:expr;

    $additional_tries:expr,
    $pair:expr,
    $procedure:expr $(,)?
  ) => {{
    let initial_try = {
      let $local_blockhash = $initial_blockhash;
      $procedure
    };
    match initial_try {
      Err(err) => {
        let inferred_additional_tries: u8 = $additional_tries;
        if let Some(n) = inferred_additional_tries.checked_sub(1) {
          let mut opt = None;
          for _ in 0..n {
            let $local_blockhash = {
              let pair_mut = &mut $pair;
              let (pkgs_aux, trans) = pair_mut.parts_mut();
              let res = trans
                .send_retrieve_and_decode_contained(
                  &mut pkgs_aux.get_latest_blockhash().data(None).build(),
                  pkgs_aux,
                )
                .await?;
              res.result?.value.blockhash
            };
            if let Ok(elem) = $procedure {
              opt = Some((elem, Some($local_blockhash)));
              break;
            }
          }
          if let Some(elem) = opt {
            Ok(elem)
          } else {
            let $local_blockhash = {
              let pair_mut = &mut $pair;
              let (pkgs_aux, trans) = pair_mut.parts_mut();
              let res = trans
                .send_retrieve_and_decode_contained(
                  &mut pkgs_aux.get_latest_blockhash().data(None).build(),
                  pkgs_aux,
                )
                .await?;
              res.result?.value.blockhash
            };
            let last = $procedure?;
            Ok((last, Some($local_blockhash)))
          }
        } else {
          Err(err)
        }
      }
      Ok(elem) => Ok((elem, None)),
    }
  }};
}

macro_rules! _create_blockchain_constants {
  (
    $address_hash_vis:vis address_hash: $address_hash:ident = $_1:literal,
    $address_hash_str_vis:vis address_hash_str: $address_hash_str:ident = $_2:literal,
    $block_hash_vis:vis block_hash: $block_hash:ident = $_3:literal,
    $block_hash_str_vis:vis block_hash_str: $block_hash_str:ident = $_4:literal,
    $signature_hash_vis:vis signature_hash: $signature_hash:ident = $_5:literal,
    $signature_hash_str_vis:vis signature_hash_str: $signature_hash_str:ident = $_6:literal,
    $transaction_hash_vis:vis transaction_hash: $transaction_hash:ident = $_7:literal,
    $transaction_hash_str_vis:vis transaction_hash_str: $transaction_hash_str:ident = $_8:literal
  ) => {
    /// Address hash as bytes
    $address_hash_vis type $address_hash = [u8; $_1];
    /// Address hash as an encoded string
    $address_hash_str_vis type $address_hash_str = ::arrayvec::ArrayString<$_2>;

    /// Block hash as bytes
    $block_hash_vis type $block_hash = [u8; $_3];
    /// Block hash as an encoded string
    $block_hash_str_vis type $block_hash_str = ::arrayvec::ArrayString<$_4>;

    /// Signature hash as bytes
    $signature_hash_vis type $signature_hash = ::cl_aux::ArrayWrapper<u8, $_5>;
    /// Signature hash as an encoded string
    $signature_hash_str_vis type $signature_hash_str = ::arrayvec::ArrayString<$_6>;

    /// Transaction hash as bytes
    $transaction_hash_vis type $transaction_hash = ::cl_aux::ArrayWrapper<u8, $_7>;
    /// Transaction hash as an encoded string
    $transaction_hash_str_vis type $transaction_hash_str = ::arrayvec::ArrayString<$_8>;
  };
}

macro_rules! _generic_api_doc {
  () => {
    "Used to group a set of packages related to this API as well as any additional instance parameters."
  };
}

macro_rules! _generic_res_data_elem_doc {
  () => {
    "Element that makes up most of the expected data response returned by the server."
  };
}
