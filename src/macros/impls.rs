macro_rules! _create_json_endpoint {
  (
    // API
    $api:ty$([$api_ty:ident])?;

    // Request
    $(#[$mac:meta])*
    $struct_name:ident<
      $( $build_lt:lifetime ),*;
      $( const $build_const:ident: $build_const_ty:ty ),*;
      $( $build_ty:ident $(<$build_path_lt:lifetime>)? $($build_path:path)* $(= $build_default:ty)? ),*
    >$(( $struct_elem:ty ))?

    // Response
    |$raw_response_ident:ident: $raw_response:ty| -> $processed_response:ty $raw_response_block:block

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
    #[doc = concat!(
      "It is preferable to call [crate::RequestManager::",
      stringify!($build_fn),
      "] instead of building this structure directly."
    )]
    #[derive(Debug, serde::Serialize)]
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
      $( $build_lt, )*
      $( const $build_const: $build_const_ty, )*
      $( $build_ty: $(for<$build_path_lt>)? $($build_path +)*, )*
    > crate::RequestData for crate::protocol::JsonRequest<$struct_name<
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

    /// If any, wraps the content provided in [Self::new].
    #[derive(Debug)]
    pub struct $params_struct<'rpd> {
      $($params_arg: $params_arg_ty,)*
      phantom: core::marker::PhantomData<&'rpd ()>
    }

    impl<'rpd> $params_struct<'rpd> {
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
      $( $build_lt, )*
      $( const $build_const: $build_const_ty, )*
      $( $build_ty: $(for<$build_path_lt>)? $($build_path +)*, )*
    > crate::RequestParams<(), ()> for crate::protocol::JsonRequest<$struct_name<
      $( $build_lt, )*
      $( $build_const, )*
      $( $build_ty, )*
    >> {
      #[inline]
      fn modify_all_params(_: &mut (), _: ()) -> crate::Result<()> {
        Ok(())
      }
    }

    impl<
      'rpd,
      $( $build_lt, )*
      $( const $build_const: $build_const_ty, )*
      $( $build_ty: $(for<$build_path_lt>)? $($build_path +)*, )*
    > crate::RequestParams<crate::network::HttpParams, $params_struct<'rpd>> for crate::protocol::JsonRequest<$struct_name<
      $( $build_lt, )*
      $( $build_const, )*
      $( $build_ty, )*
    >> {
      #[inline]
      fn modify_all_params(cp: &mut crate::network::HttpParams, $params_struct { $($params_arg,)* .. }: $params_struct<'rpd>) -> crate::Result<()> {
        let $params_cp = cp;
        $params_block;
        Ok(())
      }
    }

    impl<CP, $($api_ty)?> crate::RequestManager<$api, CP> {
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
          crate::protocol::JsonRequest<$struct_name<
            $( $build_lt, )*
            $( $build_const, )*
            $( $build_ty, )*
          >>
        $($req_close)?
      {
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
    #[doc = concat!(
      "It is preferable to call [crate::RequestManager::",
      stringify!($build_fn),
      "] instead of building this structure directly."
    )]
    #[derive(Debug, serde::Serialize)]
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
    > crate::RequestData for crate::protocol::JsonRpcRequest<$struct_name<
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
        rr._into_processed::<(), _, _, Self, _>(|raw| {
          let $raw_response_ident = raw;
          $raw_response_block
        })
      }
    }

    impl<
      $( $build_lt $(: $build_lt_bound)?, )*
      $( const $build_const: $build_const_ty, )*
      $( $build_ty: $(for<$build_path_lt>)? $($build_path +)* $($build_lf)? , )*
    > crate::RequestParams<(), ()> for crate::protocol::JsonRpcRequest<$struct_name<
      $( $build_lt, )*
      $( $build_const, )*
      $( $build_ty, )*
    >> {
      #[inline]
      fn modify_all_params(_: &mut (), _: ()) -> crate::Result<()> {
        Ok(())
      }
    }

    impl<
      $( $build_lt $(: $build_lt_bound)?, )*
      $( const $build_const: $build_const_ty, )*
      $( $build_ty: $(for<$build_path_lt>)? $($build_path +)* $($build_lf)? , )*
    > crate::RequestParams<crate::network::HttpParams, ()> for crate::protocol::JsonRpcRequest<$struct_name<
      $( $build_lt, )*
      $( $build_const, )*
      $( $build_ty, )*
    >> {
      #[inline]
      fn modify_all_params(hp: &mut crate::network::HttpParams, _: ()) -> crate::Result<()> {
        hp._method = crate::network::HttpMethod::_Post;
        Ok(())
      }
    }

    impl<
      $( $build_lt $(: $build_lt_bound)?, )*
      $( const $build_const: $build_const_ty, )*
      $( $build_ty: $(for<$build_path_lt>)? $($build_path +)* $($build_lf)? , )*
    > crate::RequestParams<
      crate::CommonParams<crate::network::HttpParams, crate::utils::RequestThrottling>,
      ()
    > for crate::protocol::JsonRpcRequest<$struct_name<
      $( $build_lt, )*
      $( $build_const, )*
      $( $build_ty, )*
    >> {
      #[inline]
      fn modify_all_params(hp: &mut crate::CommonParams<crate::network::HttpParams, crate::utils::RequestThrottling>, _: ()) -> crate::Result<()> {
        hp.tp._method = crate::network::HttpMethod::_Post;
        Ok(())
      }
    }

    impl<
      $( $build_lt $(: $build_lt_bound)?, )*
      $( const $build_const: $build_const_ty, )*
      $( $build_ty: $(for<$build_path_lt>)? $($build_path +)* $($build_lf)? , )*
    > crate::RequestParams<crate::network::WsParams, ()> for crate::protocol::JsonRpcRequest<$struct_name<
      $( $build_lt, )*
      $( $build_const, )*
      $( $build_ty, )*
    >> {
      #[inline]
      fn modify_all_params(_: &mut crate::network::WsParams, _: ()) -> crate::Result<()> {
        Ok(())
      }
    }

    impl<CP> crate::RequestManager<$api, CP> {
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
          crate::protocol::JsonRpcRequest<$struct_name<
            $( $build_lt, )*
            $( $build_const, )*
            $( $build_ty, )*
          >>
        $($req_close)?
      {
        let rslt = self._json_rpc_request($method_name, $build_params);
        $( let rslt = $build_rslt(rslt); )?
        rslt
      }
    }
  };
}
