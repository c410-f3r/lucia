// Protocols

macro_rules! _create_json_endpoint {
  (
    // API
    $api:ty;

    // Request
    $(#[$mac:meta])*
    $struct_name:ident<
      $( $build_lt:lifetime ),*;
      $( const $build_const:ident: $build_const_ty:ty ),*;
      $( $build_ty:ident $(<$build_path_lt:lifetime>)? $($build_path:path)* $(= $build_default:ty)? ),*
    >$(( $struct_elem:ty ))?

    // Response
    |$raw_response_ident:ident: $raw_response:ty| -> $processed_response:ty $raw_response_block:block

    // Blockchain calling method
    $( #[$build_doc:meta] )*
    $build_fn:ident(
      $($build_arg:ident: $build_arg_ty:ty),* $(,)?
    ) $(-> crate::Result$req_open:tt)? $(: $req_close:tt)? {
      |$build_api:ident, $build_trans_params:ident| $build_block:block
    }

    // Optional calling method mapper
    $($build_rslt:expr)?
  ) => {
    #[doc = concat!(
      "Not meant to be called directly. See [crate::RequestBuilder::",
      stringify!($build_fn),
      "]"
    )]
    #[derive(Debug, Eq, PartialEq, serde::Serialize)]
    $(#[$mac])*
    pub struct $struct_name<
      $( $build_lt, )*
      $( const $build_const: $build_const_ty, )*
      $( $build_ty: $(for<$build_path_lt>)? $($build_path +)* $(= $build_default)?, )*
    >$( ( pub $struct_elem ) )?;

    impl<
      $( $build_lt, )*
      $( const $build_const: $build_const_ty, )*
      $( $build_ty: $(for<$build_path_lt>)? $($build_path +)*, )*
    > crate::Request for crate::protocol::JsonRequest<$struct_name<
      $( $build_lt, )*
      $( $build_const, )*
      $( $build_ty, )*
    >> {
      type ProcessedResponse = $processed_response;
      type RawResponse = $raw_response;

      #[inline]
      fn id(&self) -> crate::types::Id {
        self.id
      }

      #[inline]
      fn process_response(raw: Self::RawResponse) -> crate::Result<Self::ProcessedResponse> {
        let fun = |$raw_response_ident: Self::RawResponse| $raw_response_block;
        fun(raw)
      }
    }

    impl crate::RequestBuilder<$api> {
      #[inline]
      pub fn $build_fn<
        $( $build_lt, )*
        $( const $build_const: $build_const_ty, )*
        $( $build_ty: $(for<$build_path_lt>)? $($build_path +)*, )*
      >(
        &mut self,
        $($build_arg: $build_arg_ty),*
      ) ->
        $(crate::Result$req_open)?
          crate::protocol::JsonRequest<$struct_name<
            $( $build_lt, )*
            $( $build_const, )*
            $( $build_ty, )*
          >>
        $($req_close)?
      {
        let $build_api = &self._api;
        let $build_trans_params = &mut self._tp;
        let req = $build_block;
        let rslt = self._json_request(req);
        $( let rslt = $build_rslt(rslt); )?
        rslt
      }
    }
  };
}

macro_rules! _create_json_rpc_endpoint {
  (
    // API
    $api:ty;

    // Request
    $(#[$mac:meta])*
    $method_name:literal => $struct_name:ident<
      $( $build_lt:lifetime $(: $build_lt_bound:lifetime)? ),*;
      $( const $build_const:ident: $build_const_ty:ty ),*;
      $( $build_ty:ident $(<$build_path_lt:lifetime>)? $($build_path:path)* $(= $build_default:ty)? ),*
    >$(( $struct_elem:ty ))?

    // Response
    |$raw_response_ident:ident: $raw_response:ty| -> $processed_response:ty $raw_response_block:block

    // Blockchain calling method
    $( #[$build_doc:meta] )*
    $build_fn:ident(
      $($build_arg:ident: $build_arg_ty:ty),* $(,)?
    ) $(-> crate::Result$req_open:tt)? $(: $req_close:tt)? $build_params:block

    // Optional calling method mapper
    $($build_rslt:expr)?
  ) => {
    #[doc = concat!(
      "Not meant to be called directly. See [crate::RequestBuilder::",
      stringify!($build_fn),
      "]"
    )]
    #[derive(Debug, Eq, PartialEq, serde::Serialize)]
    $(#[$mac])*
    pub struct $struct_name<
      $( $build_lt, )*
      $( const $build_const: $build_const_ty, )*
      $( $build_ty: $(for<$build_path_lt>)? $($build_path +)* $(= $build_default)?, )*
    >$( ( pub $struct_elem ) )?;

    impl<
      $( $build_lt $(: $build_lt_bound)?, )*
      $( const $build_const: $build_const_ty, )*
      $( $build_ty: $(for<$build_path_lt>)? $($build_path +)*, )*
    > crate::Request for crate::protocol::JsonRpcRequest<$struct_name<
      $( $build_lt, )*
      $( $build_const, )*
      $( $build_ty, )*
    >> {
      type ProcessedResponse = crate::protocol::ProcessedJsonRpcResponse<$processed_response>;
      type RawResponse = crate::protocol::JsonRpcResponse<$raw_response>;

      #[inline]
      fn id(&self) -> crate::types::Id {
        self.id
      }

      #[inline]
      fn process_response(rr: Self::RawResponse) -> crate::Result<Self::ProcessedResponse> {
        rr._into_processed::<Self, _>(|raw| {
          let $raw_response_ident = raw;
          $raw_response_block
        })
      }
    }

    impl crate::RequestBuilder<$api> {
      #[inline]
      pub fn $build_fn<
        $( $build_lt $(: $build_lt_bound)?, )*
        $( const $build_const: $build_const_ty, )*
        $( $build_ty: $(for<$build_path_lt>)? $($build_path +)*, )*
      >(
        &mut self,
        $($build_arg: $build_arg_ty),*
      ) ->
        $(crate::Result$req_open)?
          crate::protocol::JsonRpcRequest<$struct_name<
            $( $build_lt, )*
            $( $build_const, )*
            $( $build_ty, )*
          >>
        $($req_close)?
      {
        self._tp._http_params._set(
          crate::network::HttpMethod::Post, self._api.rt, crate::api::Api::origin(&self._api)
        );
        let rslt = self._json_rpc_request($method_name, $build_params);
        $( let rslt = $build_rslt(rslt); )?
        rslt
      }
    }
  };
}

// Etc

/// `ArrayVec` is ubiquitous in this project so it is natural to provide a constructor
/// similar to `vec![]`.
#[macro_export]
macro_rules! arrayvec {
  ($($x:expr),* $(,)?) => {(|| {
    #[allow(unused_mut)]
    let mut vec = arrayvec::ArrayVec::new();
    $( vec.try_push($x)?; )*
    $crate::Result::Ok(vec)
  })()};
  ($x:expr; $n:expr) => {(|| {
    let mut vec = arrayvec::ArrayVec::<_, $n>::new();
    for _ in 0..$n {
      $( vec.try_push($x)?; )*
    }
    $crate::Result::Ok(vec)
  })()}
}

macro_rules! _create_array_string_type {
  ($vis:vis $name:ident = $n:expr) => {
    #[allow(unused_qualifications)]
    $vis type $name = arrayvec::ArrayString<$n>;
  };
}

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
    _create_byte_array_type!(pub $address_hash = $_1);
    _create_byte_array_type!(pub $block_hash = $_2);
    _create_byte_array_wrapper_type!(pub $signature_hash = $_3);
    _create_byte_array_wrapper_type!(pub $transaction_hash = $_4);

    _create_array_string_type!(pub $address_hash_str = $_5);
    _create_array_string_type!(pub $block_hash_str = $_6);
    _create_array_string_type!(pub $signature_hash_str = $_7);
    _create_array_string_type!(pub $transaction_hash_str = $_8);
  };
}

macro_rules! _create_byte_array_type {
  ($vis:vis $name:ident = $n:literal) => {
    $vis type $name = [u8; $n];
  };
}

macro_rules! _create_byte_array_wrapper_type {
  ($vis:vis $name:ident = $n:literal) => {
    #[allow(unused)]
    $vis type $name = cl_aux::ArrayWrapper<u8, $n>;
  };
}

macro_rules! _create_generic_test {
  ($executor:ident, $test:ident, $pair:expr, $parts_cb:expr, $rslt_cb:expr) => {
    #[$executor::test]
    async fn $test() {
      fn parts_cb_infer<'pair, A, O, T>(
        rb: &'pair mut $crate::RequestBuilder<A>,
        trans: &'pair mut T,
        cb: impl FnOnce(&'pair mut $crate::RequestBuilder<A>, &'pair mut T) -> O,
      ) -> O {
        cb(rb, trans)
      }
      fn rslt_cb_infer<'pair, A, O, R, T>(
        rb: &'pair mut $crate::RequestBuilder<A>,
        trans: &'pair mut T,
        rslt: R,
        cb: impl FnOnce(&'pair mut $crate::RequestBuilder<A>, &'pair mut T, R) -> O,
      ) -> O {
        cb(rb, trans, rslt)
      }
      crate::utils::_init_tracing();
      let mut pair = $pair;
      let (rb, trans) = pair.parts_mut();
      let rslt = parts_cb_infer(rb, trans, $parts_cb).await;
      rslt_cb_infer(rb, trans, rslt, $rslt_cb).await;
    }
  };
}

macro_rules! _create_http_test {
  ($api:expr, $test:ident, $cb:expr) => {
    mod $test {
      use super::*;

      #[cfg(feature = "reqwest")]
      _create_generic_test! {
        tokio,
        reqwest,
        crate::Pair::new(reqwest::Client::default(), $api),
        $cb,
        |_, _, _| async {}
      }

      #[cfg(feature = "surf")]
      _create_generic_test! {
        async_std,
        surf,
        crate::Pair::new(surf::Client::default(), $api),
        $cb,
        |_, _, _| async {}
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
  ($api:expr, $sub:ident, ($($unsub:ident),+), $cb:expr) => {
    #[cfg(feature = "tokio-tungstenite")]
    _create_generic_test! {
      tokio,
      $sub,
      crate::Pair::new(crate::network::tokio_tungstenite(&$api).await.unwrap(), $api),
      $cb,
      |rb, trans, subs| async move {
        let mut iter = subs.into_iter();
        let _ = trans.send(
          ($( rb.$unsub(iter.next().unwrap()), )+),
          rb.tp_mut()
        ).await.unwrap();
      }
    }
  };
}

macro_rules! _debug {
  ($($tt:tt)+) => {
    #[cfg(feature = "tracing")]
    tracing::debug!($($tt)+);
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
    $pair_mut:expr,
    $procedure:expr $(,)?
  ) => {{
    let fun = || async move {
      async fn additional_blockhash<T>(
        pair: &mut $crate::Pair<$crate::api::blockchain::solana::Solana, T>,
      ) -> $crate::Result<SolanaAddressHash>
      where
        T: Send + Transport,
      {
        let (rb, trans) = pair.parts_mut();
        let req = rb.get_latest_blockhash(None);
        let res = trans.send_retrieve_and_decode_one(&req, rb.tp_mut()).await?;
        Ok(res.result.value.blockhash)
      }
      let initial_try = {
        let $local_blockhash = $initial_blockhash;
        $procedure
      };
      match initial_try {
        Err(err) => {
          let inferred_additional_tries: u8 = $additional_tries;
          let n = if let Some(elem) = inferred_additional_tries.checked_sub(1) {
            elem
          } else {
            return Err(err);
          };
          for _ in 0..n {
            let $local_blockhash = additional_blockhash($pair_mut).await?;
            if let Ok(elem) = $procedure {
              return Ok((elem, Some($local_blockhash)));
            }
          }
          let $local_blockhash = additional_blockhash($pair_mut).await?;
          let elem = $procedure?;
          Ok((elem, Some($local_blockhash)))
        }
        Ok(elem) => Ok((elem, None)),
      }
    };
    fun()
  }};
}
