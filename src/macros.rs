#[macro_use]
mod impls;

macro_rules! _create_blockchain_constants {
  (
    address_hash: $address_hash:ident = $_1:literal,
    block_hash: $block_hash:ident = $_2:literal,
    signature_hash: $signature_hash:ident = $_3:literal,
    transaction_hash: $transaction_hash:ident = $_4:literal,

    address_hash_str: $address_hash_str:ident = $_5:literal,
    block_hash_str: $block_hash_str:ident = $_6:literal,
    signature_hash_str: $signature_hash_str:ident = $_7:literal,
    transaction_hash_str: $transaction_hash_str:ident = $_8:literal
  ) => {
    /// Address hash as bytes
    #[allow(unused_qualifications)]
    pub type $address_hash = [u8; $_1];
    /// Block hash as bytes
    #[allow(unused_qualifications)]
    pub type $block_hash = [u8; $_2];
    /// Signature hash as bytes
    #[allow(unused_qualifications)]
    pub type $signature_hash = cl_aux::ArrayWrapper<u8, $_3>;
    /// Transaction hash as bytes
    #[allow(unused_qualifications)]
    pub type $transaction_hash = cl_aux::ArrayWrapper<u8, $_4>;

    /// Address hash as an encoded string
    #[allow(unused_qualifications)]
    pub type $address_hash_str = arrayvec::ArrayString<$_5>;
    /// Block hash as an encoded string
    #[allow(unused_qualifications)]
    pub type $block_hash_str = arrayvec::ArrayString<$_6>;
    /// Signature hash as an encoded string
    #[allow(unused_qualifications)]
    pub type $signature_hash_str = arrayvec::ArrayString<$_7>;
    /// Transaction hash as an encoded string
    #[allow(unused_qualifications)]
    pub type $transaction_hash_str = arrayvec::ArrayString<$_8>;
  };
}

macro_rules! _create_generic_test {
  ($executor:ident, $test:ident, $pair:expr, $parts_cb:expr, $rslt_cb:expr $(, $(#[$attrs:meta])+)?) => {
    $($(#[$attrs])+)?
    #[$executor::test]
    async fn $test() {
      fn parts_cb_infer<'pair, A, CP, DRSR, O, T>(
        rm: &'pair mut $crate::RequestManager<A, CP, DRSR>,
        trans: &'pair mut T,
        cb: impl FnOnce(&'pair mut $crate::RequestManager<A, CP, DRSR>, &'pair mut T) -> O,
      ) -> O {
        cb(rm, trans)
      }
      fn rslt_cb_infer<'pair, A, CP, DRSR, O, R, T>(
        rm: &'pair mut $crate::RequestManager<A, CP, DRSR>,
        trans: &'pair mut T,
        rslt: R,
        cb: impl FnOnce(&'pair mut $crate::RequestManager<A, CP, DRSR>, &'pair mut T, R) -> O,
      ) -> O {
        cb(rm, trans, rslt)
      }
      crate::utils::_init_tracing();
      let mut pair = $pair;
      let (rm, trans) = pair.parts_mut();
      let rslt = parts_cb_infer(rm, trans, $parts_cb).await;
      rslt_cb_infer(rm, trans, rslt, $rslt_cb).await;
    }
  };
}

macro_rules! _create_http_test {
  ($cp_drsr:expr, $test:ident, $cb:expr $(, $(#[$attrs:meta])+)?) => {
    mod $test {
      use super::*;

      #[cfg(feature = "reqwest")]
      _create_generic_test! {
        tokio,
        reqwest,
        {
          let (cp, drsr) = $cp_drsr;
          crate::Pair::new(
            crate::RequestManager::new(<_>::default(), cp, drsr),
            reqwest::Client::default(),
          )
        },
        $cb,
        |_, _, _| async {}
        $(, $(#[$attrs])+)?
      }

      #[cfg(feature = "surf")]
      _create_generic_test! {
        async_std,
        surf,
        {
          let (cp, drsr) = $cp_drsr;
          crate::Pair::new(
            crate::RequestManager::new(<_>::default(), cp, drsr),
            surf::Client::default(),
          )
        },
        $cb,
        |_, _, _| async {}
        $(, $(#[$attrs])+)?
      }
    }
  };
}

macro_rules! _create_set_of_request_throttling {
  (
    $name:ident {
      $( $method:ident ),+ $(,)?
    }
  ) => {
    /// A set of [$crate::utils::RequestThrottling] for specified API usage
    #[derive(Debug)]
    pub struct $name {
      $(
        pub(crate) $method: $crate::utils::RequestThrottling,
      )+
    }

    impl $name {
      #[inline]
      pub fn new(
        $( $method: $crate::utils::RequestLimit, )+
      ) -> Self {
        Self {
          $(
            $method: $crate::utils::RequestThrottling::from_rl($method),
          )+
        }
      }
    }
  };
}

macro_rules! _create_tokio_tungstenite_test {
  ($cp_drsr:expr, $sub:ident, ($($unsub:ident),+), $cb:expr $(, $(#[$attrs:meta])+)?) => {
    #[cfg(feature = "tokio-tungstenite")]
    _create_generic_test! {
      tokio,
      $sub,
      {
        let (cp, drsr) = $cp_drsr;
        let trans = crate::network::tokio_tungstenite(&cp.tp).await.unwrap();
        crate::Pair::new(crate::RequestManager::new(<_>::default(), cp, drsr), trans)
      },
      $cb,
      |rm, trans, subs| async move {
        let mut iter = subs.into_iter();
        let ids = [$( rm.$unsub(iter.next().unwrap()), )+];
        let _ = trans.send(rm, &ids, ()).await.unwrap();
      }
      $(, $(#[$attrs])+)?
    }
  };
}

macro_rules! _debug {
  ($($tt:tt)+) => {
    #[cfg(feature = "tracing")]
    tracing::debug!($($tt)+);
  };
}

macro_rules! generic_data_format_doc {
  ($ty:literal) => {
    concat!(
      "Internal wrapper used in every generic ",
      $ty,
      " to manage different internal implementations."
    )
  };
}

/// Implements `Serialize` for several collections
///
/// Must be in sync with `requests.rs`.
macro_rules! _impl_se_collections {
  (
    for $drsr:ty => $bound:path;

    $( array: |$array_self:ident, $array_bytes:ident, $array_drsr:ident| $array_block:block )?
    $( arrayvec: |$arrayvec_self:ident, $arrayvec_bytes:ident, $arrayvec_drsr:ident| $arrayvec_block:block )?
    slice: |$slice_self:ident, $slice_bytes:ident, $slice_drsr:ident| $slice_block:block
    slice_ref: |$slice_ref_self:ident, $slice_ref_bytes:ident, $slice_ref_drsr:ident| $slice_ref_block:block
    vec: |$vec_self:ident, $vec_bytes:ident, $vec_drsr:ident| $vec_block:block
  ) => {
    $(
      impl<T, const N: usize> crate::dnsn::Serialize<$drsr> for [T; N]
      where
        T: $bound,
      {
        #[inline]
        fn to_bytes<B>(&self, bytes: &mut B, drsr: &mut $drsr) -> crate::Result<()>
        where
          B: crate::utils::ByteBuffer,
        {
          let $array_self = self;
          let $array_bytes = bytes;
          let $array_drsr = drsr;
          $array_block;
          Ok(())
        }
      }
    )?

    $(
      impl<T, const N: usize> crate::dnsn::Serialize<$drsr> for arrayvec::ArrayVec<T, N>
      where
        T: $bound,
      {
        #[inline]
        fn to_bytes<B>(&self, bytes: &mut B, drsr: &mut $drsr) -> crate::Result<()>
        where
          B: crate::utils::ByteBuffer,
        {
          let $arrayvec_self = self;
          let $arrayvec_bytes = bytes;
          let $arrayvec_drsr = drsr;
          $arrayvec_block;
          Ok(())
        }
      }
    )?

    impl<T> crate::dnsn::Serialize<$drsr> for [T]
    where
      T: $bound,
    {
      #[inline]
      fn to_bytes<B>(&self, bytes: &mut B, drsr: &mut $drsr) -> crate::Result<()>
      where
        B: crate::utils::ByteBuffer,
      {
        let $slice_self = self;
        let $slice_bytes = bytes;
        let $slice_drsr = drsr;
        $slice_block;
        Ok(())
      }
    }

    impl<T> crate::dnsn::Serialize<$drsr> for &'_ [T]
    where
      T: $bound,
    {
      #[inline]
      fn to_bytes<B>(&self, bytes: &mut B, drsr: &mut $drsr) -> crate::Result<()>
      where
        B: crate::utils::ByteBuffer,
      {
        let $slice_ref_self = self;
        let $slice_ref_bytes = bytes;
        let $slice_ref_drsr = drsr;
        $slice_ref_block;
        Ok(())
      }
    }

    impl<T> crate::dnsn::Serialize<$drsr> for Vec<T>
    where
      T: $bound,
    {
      #[inline]
      fn to_bytes<B>(&self, bytes: &mut B, drsr: &mut $drsr) -> crate::Result<()>
      where
        B: crate::utils::ByteBuffer,
      {
        let $vec_self = self;
        let $vec_bytes = bytes;
        let $vec_drsr = drsr;
        $vec_block;
        Ok(())
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
              let (rm, trans) = pair_mut.parts_mut();
              let req = rm.get_latest_blockhash(None);
              let res = trans.send_retrieve_and_decode_one(rm, &req, ()).await?;
              res.result.value.blockhash
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
              let (rm, trans) = pair_mut.parts_mut();
              let req = rm.get_latest_blockhash(None);
              let res = trans.send_retrieve_and_decode_one(rm, &req, ()).await?;
              res.result.value.blockhash
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
