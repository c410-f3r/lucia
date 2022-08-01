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
      "It is preferable to call [crate::RequestManager::",
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
      RESP,
      $( $build_lt, )*
      $( const $build_const: $build_const_ty, )*
      $( $build_ty: $(for<$build_path_lt>)? $($build_path +)*, )*
    > crate::RequestResponse<RESP> for $ser<$struct_name<
      $( $build_lt, )*
      $( $build_const, )*
      $( $build_ty, )*
    >>
    where
      for<'any> &'any RESP: Into<&'any crate::network::http::Response>
    {
      type ProcessedResponse = $processed_response;
      type RawResponse = $der<$raw_response>;

      #[inline]
      fn process(raw: Self::RawResponse, resp: &RESP) -> crate::Result<Self::ProcessedResponse> {
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
    > crate::RequestParamsModifier<
      crate::CommonParams<crate::network::http::ReqParams, UP>,
      $params_struct<'reqp>
    > for $ser<$struct_name<
      $( $build_lt, )*
      $( $build_const, )*
      $( $build_ty, )*
    >> {
      #[inline]
      fn modify_all_params(
        cp: &mut crate::CommonParams<crate::network::http::ReqParams, UP>,
        $params_struct { $($params_arg,)* .. }: $params_struct<'reqp>
      ) -> crate::Result<()> {
        let $params_cp = cp;
        $params_block;
        Ok(())
      }
    }

    impl<CP, DRSR, $($api_ty)?> crate::RequestManager<$api, CP, DRSR> {
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
      "It is preferable to call [crate::RequestManager::",
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
      RESP
    > crate::RequestResponse<RESP> for crate::data_format::JsonRpcRequest<$struct_name<
      $( $build_lt, )*
      $( $build_const, )*
      $( $build_ty, )*
    >> {
      type ProcessedResponse = crate::data_format::ProcessedJsonRpcResponse<$processed_response>;
      type RawResponse = crate::data_format::JsonRpcResponse<$raw_response>;

      #[inline]
      fn process(rr: Self::RawResponse, _: &RESP) -> crate::Result<Self::ProcessedResponse> {
        rr._into_processed(|raw| {
          let $raw_response_ident = raw;
          $raw_response_block
        })
      }
    }

    impl<
      $( $build_lt $(: $build_lt_bound)?, )*
      $( const $build_const: $build_const_ty, )*
      $( $build_ty: $(for<$build_path_lt>)? $($build_path +)* $($build_lf)? , )*
    > crate::RequestParamsModifier<
      crate::CommonParams<(), ()>,
      ()
    > for crate::data_format::JsonRpcRequest<$struct_name<
      $( $build_lt, )*
      $( $build_const, )*
      $( $build_ty, )*
    >>
    {
      #[inline]
      fn modify_all_params(_: &mut crate::CommonParams<(), ()>, _: ()) -> crate::Result<()> {
        Ok(())
      }
    }

    impl<
      $( $build_lt $(: $build_lt_bound)?, )*
      $( const $build_const: $build_const_ty, )*
      $( $build_ty: $(for<$build_path_lt>)? $($build_path +)* $($build_lf)? , )*
    > crate::RequestParamsModifier<
      crate::CommonParams<crate::network::http::ReqParams, ()>,
      ()
    > for crate::data_format::JsonRpcRequest<$struct_name<
      $( $build_lt, )*
      $( $build_const, )*
      $( $build_ty, )*
    >>
    {
      #[inline]
      fn modify_all_params(hp: &mut crate::CommonParams<crate::network::http::ReqParams, ()>, _: ()) -> crate::Result<()> {
        hp.tp._method = crate::network::http::Method::Post;
        hp.tp._mime_type = Some(crate::network::http::MimeType::_Json);
        Ok(())
      }
    }

    impl<
      $( $build_lt $(: $build_lt_bound)?, )*
      $( const $build_const: $build_const_ty, )*
      $( $build_ty: $(for<$build_path_lt>)? $($build_path +)* $($build_lf)? , )*
    > crate::RequestParamsModifier<
      crate::CommonParams<crate::network::http::ReqParams, crate::utils::RequestThrottling>,
      ()
    > for crate::data_format::JsonRpcRequest<$struct_name<
      $( $build_lt, )*
      $( $build_const, )*
      $( $build_ty, )*
    >>
    {
      #[inline]
      fn modify_all_params(hp: &mut crate::CommonParams<crate::network::http::ReqParams, crate::utils::RequestThrottling>, _: ()) -> crate::Result<()> {
        hp.tp._mime_type = Some(crate::network::http::MimeType::_Json);
        hp.tp._method = crate::network::http::Method::Post;
        Ok(())
      }
    }

    impl<
      $( $build_lt $(: $build_lt_bound)?, )*
      $( const $build_const: $build_const_ty, )*
      $( $build_ty: $(for<$build_path_lt>)? $($build_path +)* $($build_lf)? , )*
    > crate::RequestParamsModifier<
      crate::CommonParams<crate::network::ws::ReqParams, ()>,
      ()
    > for crate::data_format::JsonRpcRequest<$struct_name<
      $( $build_lt, )*
      $( $build_const, )*
      $( $build_ty, )*
    >>
    {
      #[inline]
      fn modify_all_params(_: &mut crate::CommonParams<crate::network::ws::ReqParams, ()>, _: ()) -> crate::Result<()> {
        Ok(())
      }
    }

    impl<CP, DRSR> crate::RequestManager<$api, CP, DRSR> {
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
          crate::data_format::JsonRpcRequest<$struct_name<
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
