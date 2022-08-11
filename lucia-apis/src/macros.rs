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

macro_rules! _create_endpoint {
  (
    // API
    $api:ty$([$api_ty:ident])? => $der:ident|$ser:ident|$ser_method:ident;

    // Request
    $(#[$mac:meta])*
    $struct_name:ident<
      $( $build_lt:lifetime ),*;
      $( const $build_const:ident: $build_const_ty:ty ),*;
      $( $build_ty:ident $(<$build_path_lt:lifetime>)? $($build_path:path)* $(= $build_default:ty)? ),*
    >$(( $struct_elem:ty ))?

    // Response
    |$raw_response_ident:ident: $raw_response:ty, $tm_ident:ident| -> $processed_response:ty $raw_response_block:block

    // Parameters
    $params_struct:ident(
      $($params_arg:ident: $params_arg_ty:ty),* $(,)?
    ) -> crate::Result<()> {
      |$params_cp:ident| $params_block:block
    }

    // Calling method
    $( #[$build_doc:meta] )*
    $build_fn:ident(
      $($build_arg:ident: $build_arg_ty:ty),* $(,)?
    ) $(-> crate::Result$req_open:tt)? $(: $req_close:tt)? {
      || $build_block:block
    }

    // Optional calling method mapper
    $($build_rslt:expr)?
  ) => {
    #[cfg_attr(feature = "serde", derive(serde::Serialize))]
    #[derive(Debug)]
    #[doc = concat!(
      "It is preferable to call [crate::misc::RequestManagerWrapper::",
      stringify!($build_fn),
      "] instead of building this structure directly."
    )]
    $(#[$mac])*
    pub struct $struct_name<
      $( $build_lt, )*
      $( const $build_const: $build_const_ty, )*
      $( $build_ty: $(for<$build_path_lt>)? $($build_path +)* $(= $build_default)?, )*
    >$( (
      /// Payload data
      pub $struct_elem
    ) )?;

    impl<
      $( $build_lt )*
      $( const $build_const: $build_const_ty, )*
      $( $build_ty: $(for<$build_path_lt>)? $($build_path +)* , )*
    > lucia::misc::FromErrorTy for $struct_name<
      $( $build_lt, )*
      $( $build_const, )*
      $( $build_ty, )*
    > {
      type Error = crate::Error;
    }

    impl<
      $( $build_lt, )*
      $( const $build_const: $build_const_ty, )*
      $( $build_ty: $(for<$build_path_lt>)? $($build_path +)*, )*
      CP,
      DRSR,
      REQP,
      RESP,
    > lucia::req_res::Request<CP, DRSR, REQP, RESP> for $struct_name<
      $( $build_lt, )*
      $( $build_const, )*
      $( $build_ty, )*
    >
    where
      Self: lucia::req_res::RequestParamsModifier<CP, REQP>,
      $der<$raw_response>: lucia::dnsn::Deserialize<DRSR>,
      $ser<Self>: lucia::dnsn::Serialize<DRSR>,
      for<'any> &'any RESP: Into<&'any lucia::network::http::Response>
    {
      type Data = $ser<Self>;
      type ProcessedResponse = $processed_response;
      type RawResponse = $der<$raw_response>;

      #[inline]
      fn data(&self) -> &Self::Data {
        todo!()
      }

      #[inline]
      fn process(raw: Self::RawResponse, resp: &RESP) -> Result<Self::ProcessedResponse, Self::Error> {
        let fun = |$raw_response_ident: $raw_response, $tm_ident: &RESP| $raw_response_block;
        fun(raw.data, resp)
      }
    }

    /// If any, wraps the content provided in [Self::new].
    #[derive(Debug)]
    pub struct $params_struct<'reqp> {
      $($params_arg: $params_arg_ty,)*
      phantom: core::marker::PhantomData<&'reqp ()>
    }

    impl<'reqp> $params_struct<'reqp> {
      /// Please see the official API's documentation to get more information about the input data
      /// of this endpoint.
      #[inline]
      pub fn new(
        $($params_arg: $params_arg_ty,)*
      ) -> Self {
        Self {
          $($params_arg,)*
          phantom: core::marker::PhantomData
        }
      }
    }

    impl<
      'reqp,
      $( $build_lt, )*
      $( const $build_const: $build_const_ty, )*
      $( $build_ty: $(for<$build_path_lt>)? $($build_path +)*, )*
      UP
    > lucia::req_res::RequestParamsModifier<
      lucia::misc::CommonParams<lucia::network::http::ReqParams, UP>,
      $params_struct<'reqp>
    > for $struct_name<
      $( $build_lt, )*
      $( $build_const, )*
      $( $build_ty, )*
    > {
      #[inline]
      fn modify_all_params(
        cp: &mut lucia::misc::CommonParams<lucia::network::http::ReqParams, UP>,
        $params_struct { $($params_arg,)* .. }: $params_struct<'reqp>
      ) -> Result<(), Self::Error> {
        let $params_cp = cp;
        $params_block;
        Ok(())
      }
    }

    impl<CP, DRSR, $($api_ty)?> crate::misc::RequestManagerWrapper<$api, CP, DRSR> {
      /// Please see the official API's documentation to get more information about this endpoint.
      #[inline]
      pub fn $build_fn<
        $( $build_lt, )*
        $( const $build_const: $build_const_ty, )*
        $( $build_ty: $(for<$build_path_lt>)? $($build_path +)*, )*
      >(
        &mut self,
        $($build_arg: $build_arg_ty,)*
      ) ->
        $(crate::Result$req_open)?
          $ser<$struct_name<
            $( $build_lt, )*
            $( $build_const, )*
            $( $build_ty, )*
          >>
        $($req_close)?
      {
        let req = $build_block;
        let rslt = self.$ser_method(req);
        $( let rslt = $build_rslt(rslt); )?
        rslt
      }
    }
  };
}

macro_rules! _create_generic_test {
  ($executor:ident, $test:ident, $pair:expr, $parts_cb:expr, $rslt_cb:expr $(, $(#[$attrs:meta])+)?) => {
    $($(#[$attrs])+)?
    #[$executor::test]
    async fn $test() {
      fn parts_cb_infer<'pair, A, CP, DRSR, O, T>(
        rm: &'pair mut crate::misc::RequestManagerWrapper<A, CP, DRSR>,
        trans: &'pair mut T,
        cb: impl FnOnce(&'pair mut crate::misc::RequestManagerWrapper<A, CP, DRSR>, &'pair mut T) -> O,
      ) -> O {
        cb(rm, trans)
      }
      fn rslt_cb_infer<'pair, A, CP, DRSR, O, R, T>(
        rm: &'pair mut crate::misc::RequestManagerWrapper<A, CP, DRSR>,
        trans: &'pair mut T,
        rslt: R,
        cb: impl FnOnce(&'pair mut crate::misc::RequestManagerWrapper<A, CP, DRSR>, &'pair mut T, R) -> O,
      ) -> O {
        cb(rm, trans, rslt)
      }
      crate::misc::_init_tracing();
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

      _create_generic_test! {
        tokio,
        reqwest,
        {
          let (cp, drsr) = $cp_drsr;
          lucia::misc::Pair::new(
            crate::misc::RequestManagerWrapper::new(<_>::default(), cp, drsr),
            reqwest::Client::default(),
          )
        },
        $cb,
        |_, _, _| async {}
        $(, $(#[$attrs])+)?
      }

      _create_generic_test! {
        async_std,
        surf,
        {
          let (cp, drsr) = $cp_drsr;
          lucia::misc::Pair::new(
            crate::misc::RequestManagerWrapper::new(<_>::default(), cp, drsr),
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

macro_rules! _create_json_rpc_endpoint {
  (
    // API
    $api:ty;

    // Request
    $(#[$mac:meta])*
    $method_name:literal => $struct_name:ident<
      $( $build_lt:lifetime $(: $build_lt_bound:lifetime)? ),*;
      $( const $build_const:ident: $build_const_ty:ty ),*;
      $( $build_ty:ident $(<$build_path_lt:lifetime>)? $($build_path:path)* $(= $build_default:ty)? $(: $build_lf:lifetime)? ),*
    >$(( $struct_elem:ty ))?

    // Response
    |$raw_response_ident:ident: $raw_response:ty| -> $processed_response:ty $raw_response_block:block

    // Calling method
    $( #[$build_doc:meta] )*
    $build_fn:ident(
      $($build_arg:ident: $build_arg_ty:ty),* $(,)?
    ) $(-> crate::Result$req_open:tt)? $(: $req_close:tt)? $build_params:block

    // Optional calling method mapper
    $($build_rslt:expr)?
  ) => {
    #[cfg_attr(feature = "serde", derive(serde::Serialize))]
    #[derive(Debug)]
    #[doc = concat!(
      "It is preferable to call [crate::misc::RequestManagerWrapper::",
      stringify!($build_fn),
      "] instead of building this structure directly."
    )]
    $(#[$mac])*
    pub struct $struct_name<
      $( $build_lt, )*
      $( const $build_const: $build_const_ty, )*
      $( $build_ty: $(for<$build_path_lt>)? $($build_path +)* $(= $build_default)?, )*
    >$( (
      /// Payload data
      pub $struct_elem
    ) )?;

    impl<
      $( $build_lt $(: $build_lt_bound)?, )*
      $( const $build_const: $build_const_ty, )*
      $( $build_ty: $(for<$build_path_lt>)? $($build_path +)* $($build_lf)? , )*
    > lucia::misc::FromErrorTy for $struct_name<
      $( $build_lt, )*
      $( $build_const, )*
      $( $build_ty, )*
    > {
      type Error = crate::Error;
    }

    impl<
      $( $build_lt $(: $build_lt_bound)?, )*
      $( const $build_const: $build_const_ty, )*
      $( $build_ty: $(for<$build_path_lt>)? $($build_path +)* $($build_lf)? , )*
      CP,
      DRSR,
      REQP,
      RESP,
    > lucia::req_res::Request<CP, DRSR, REQP, RESP> for $struct_name<
      $( $build_lt, )*
      $( $build_const, )*
      $( $build_ty, )*
    >
    where
      lucia::data_formats::JsonRpcRequest<Self>: lucia::dnsn::Serialize<DRSR>,
      lucia::data_formats::JsonRpcResponse<$raw_response>: lucia::dnsn::Deserialize<DRSR>,
      Self: lucia::req_res::RequestParamsModifier<CP, REQP>,
    {
      type Data = lucia::data_formats::JsonRpcRequest<Self>;
      type ProcessedResponse = lucia::data_formats::ProcessedJsonRpcResponse<$processed_response>;
      type RawResponse = lucia::data_formats::JsonRpcResponse<$raw_response>;

      #[inline]
      fn data(&self) -> &Self::Data {
        todo!()
      }

      #[inline]
      fn process(rr: Self::RawResponse, _: &RESP) -> Result<Self::ProcessedResponse, Self::Error> {
        Ok(rr.into_processed(|raw| {
          let $raw_response_ident: $raw_response = raw;
          $raw_response_block
        })?)
      }
    }

    impl<
      $( $build_lt $(: $build_lt_bound)?, )*
      $( const $build_const: $build_const_ty, )*
      $( $build_ty: $(for<$build_path_lt>)? $($build_path +)* $($build_lf)? , )*
    > lucia::req_res::RequestParamsModifier<
    lucia::misc::CommonParams<(), ()>,
      ()
    > for $struct_name<
      $( $build_lt, )*
      $( $build_const, )*
      $( $build_ty, )*
    >
    {
      #[inline]
      fn modify_all_params(_: &mut lucia::misc::CommonParams<(), ()>, _: ()) -> Result<(), Self::Error> {
        Ok(())
      }
    }

    impl<
      $( $build_lt $(: $build_lt_bound)?, )*
      $( const $build_const: $build_const_ty, )*
      $( $build_ty: $(for<$build_path_lt>)? $($build_path +)* $($build_lf)? , )*
    > lucia::req_res::RequestParamsModifier<
      lucia::misc::CommonParams<lucia::network::http::ReqParams, ()>,
      ()
    > for $struct_name<
      $( $build_lt, )*
      $( $build_const, )*
      $( $build_ty, )*
    >
    {
      #[inline]
      fn modify_all_params(hp: &mut lucia::misc::CommonParams<lucia::network::http::ReqParams, ()>, _: ()) -> Result<(), Self::Error> {
        hp.tp.method = lucia::network::http::Method::Post;
        hp.tp.mime_type = Some(lucia::network::http::MimeType::_Json);
        Ok(())
      }
    }

    impl<
      $( $build_lt $(: $build_lt_bound)?, )*
      $( const $build_const: $build_const_ty, )*
      $( $build_ty: $(for<$build_path_lt>)? $($build_path +)* $($build_lf)? , )*
    > lucia::req_res::RequestParamsModifier<
    lucia::misc::CommonParams<lucia::network::http::ReqParams, lucia::misc::RequestThrottling>,
      ()
    > for $struct_name<
      $( $build_lt, )*
      $( $build_const, )*
      $( $build_ty, )*
    >
    {
      #[inline]
      fn modify_all_params(
        hp: &mut lucia::misc::CommonParams<lucia::network::http::ReqParams,
        lucia::misc::RequestThrottling>,
        _: ()
      ) -> Result<(), Self::Error> {
        hp.tp.mime_type = Some(lucia::network::http::MimeType::_Json);
        hp.tp.method = lucia::network::http::Method::Post;
        Ok(())
      }
    }

    impl<
      $( $build_lt $(: $build_lt_bound)?, )*
      $( const $build_const: $build_const_ty, )*
      $( $build_ty: $(for<$build_path_lt>)? $($build_path +)* $($build_lf)? , )*
    > lucia::req_res::RequestParamsModifier<
      lucia::misc::CommonParams<lucia::network::ws::ReqParams, ()>,
      ()
    > for $struct_name<
      $( $build_lt, )*
      $( $build_const, )*
      $( $build_ty, )*
    >
    {
      #[inline]
      fn modify_all_params(_: &mut lucia::misc::CommonParams<lucia::network::ws::ReqParams, ()>, _: ()) -> Result<(), Self::Error> {
        Ok(())
      }
    }

    impl<CP, DRSR> crate::misc::RequestManagerWrapper<$api, CP, DRSR> {
      /// Please see the official API's documentation to get more information about the expected
      /// request data.
      #[inline]
      pub fn $build_fn<
        $( $build_lt $(: $build_lt_bound)?, )*
        $( const $build_const: $build_const_ty, )*
        $( $build_ty: $(for<$build_path_lt>)? $($build_path +)* $($build_lf)? , )*
      >(
        &mut self,
        $($build_arg: $build_arg_ty),*
      ) ->
        $(crate::Result$req_open)?
          lucia::data_formats::JsonRpcRequest<$struct_name<
            $( $build_lt, )*
            $( $build_const, )*
            $( $build_ty, )*
          >>
        $($req_close)?
      {
        let rslt = self.json_rpc_request($method_name, $build_params);
        $( let rslt = $build_rslt(rslt); )?
        rslt
      }
    }
  };
}

macro_rules! _create_tokio_tungstenite_test {
  ($cp_drsr:expr, $sub:ident, ($($unsub:ident),+), $cb:expr $(, $(#[$attrs:meta])+)?) => {
    _create_generic_test! {
      tokio,
      $sub,
      {
        let (cp, drsr) = $cp_drsr;
        let trans = lucia::network::tokio_tungstenite(&cp.tp).await.unwrap();
        lucia::misc::Pair::new(crate::misc::RequestManagerWrapper::new(<_>::default(), cp, drsr), trans)
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
