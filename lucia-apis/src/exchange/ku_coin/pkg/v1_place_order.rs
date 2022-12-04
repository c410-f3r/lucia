#[lucia_macros::pkg(api(KuCoin), data_format(json), error(crate::Error), transport(http))]
pub(crate) mod pkg {
  use crate::{
    exchange::ku_coin::{
      KuCoin, KuCoinHttpPackagesAux, OrderSide, OrderStp, OrderTimeInForce, OrderType,
      ResponseWrapper,
    },
    misc::into_rslt,
  };
  use arrayvec::ArrayString;
  use lucia::{
    misc::SyncDynDebugDisplay,
    network::{HttpMethod, HttpReqParams},
  };

  #[pkg::aux]
  impl<DRSR> KuCoinHttpPackagesAux<DRSR> {}

  #[pkg::before_sending]
  async fn before_sending(
    api: &mut KuCoin,
    req_bytes: &[u8],
    req_params: &mut HttpReqParams,
  ) -> crate::Result<()> {
    api.orders_rt.rc.update_params(&api.orders_rt.rl).await?;
    req_params.method = HttpMethod::Post;
    req_params.url.push_path(format_args!("/api/v1/orders"))?;
    into_rslt(api.credentials.as_mut())?.push_headers(req_bytes, req_params)?;
    Ok(())
  }

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
  #[derive(Debug)]
  #[pkg::req_data]
  pub struct V1PlaceOrderReq<'any> {
    #[cfg_attr(feature = "serde", serde(flatten))]
    common: V1PlaceOrderCommon<'any>,
    #[cfg_attr(feature = "serde", serde(flatten, skip_serializing_if = "Option::is_none"))]
    limit: Option<V1PlaceOrderLimit<'any>>,
    #[cfg_attr(feature = "serde", serde(flatten, skip_serializing_if = "Option::is_none"))]
    market: Option<V1PlaceOrderMarket<'any>>,
  }

  #[pkg::res_data]
  pub type V1PlaceOrderRes = ResponseWrapper<V1PlaceOrderResElem>;

  #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
  #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
  #[derive(Debug)]
  #[doc = _generic_res_data_elem_doc!()]
  pub struct V1PlaceOrderResElem {
    /// Returned unique id.
    pub order_id: ArrayString<32>,
  }

  /// Common parameters used by limit and market orders
  #[derive(Debug)]
  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
  pub struct V1PlaceOrderCommon<'any> {
    /// Custom order id.
    pub client_oid: &'any SyncDynDebugDisplay,
    /// Metadata.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub remark: Option<&'any str>,
    /// Side.
    pub side: OrderSide,
    /// Self trade prevention.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub stp: Option<OrderStp>,
    /// Pair of two assets like BTC-USDT.
    pub symbol: &'any str,
    /// Type.
    pub r#type: OrderType,
  }

  /// Limit order parameters
  #[derive(Debug)]
  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
  pub struct V1PlaceOrderLimit<'any> {
    /// Cancels after the given seconds. Requires `time_in_force` to be [V1PlaceTimeInForce::GTT].
    cancel_after: u32,
    /// If order will or will not be displayed in the order book.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    hidden: Option<bool>,
    /// Only a portion of the order is displayed in the order book
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    iceberg: Option<bool>,
    /// Invalid when timeInForce is [V1PlaceTimeInForce::IOC] or [V1PlaceTimeInForce::FOK].
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    post_only: Option<bool>,
    /// Quote asset price.
    price: &'any SyncDynDebugDisplay,
    /// Base asset amount.
    size: &'any SyncDynDebugDisplay,
    /// Time in force.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    time_in_force: Option<OrderTimeInForce>,
    /// The maximum visible size of an iceberg order.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    visible_size: Option<&'any SyncDynDebugDisplay>,
  }

  /// Market order parameters. Requires one of the two optional values.
  #[derive(Debug)]
  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
  pub struct V1PlaceOrderMarket<'any> {
    /// Quote asset amount
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub funds: Option<&'any SyncDynDebugDisplay>,
    /// Base asset amount
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub size: Option<&'any SyncDynDebugDisplay>,
  }
}
