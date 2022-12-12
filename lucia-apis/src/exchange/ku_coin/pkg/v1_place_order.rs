#[lucia_macros::pkg(api(KuCoin), data_format(json), error(crate::Error), transport(http))]
pub(crate) mod pkg {
  use crate::{
    exchange::ku_coin::{
      HttpResWrapper, KuCoin, KuCoinHttpPkgsAux, OrderSide, OrderStp, OrderTimeInForce, OrderType,
    },
    misc::into_rslt,
  };
  use arrayvec::ArrayString;
  use lucia::{
    misc::SyncDynDebugDisplay,
    network::{HttpMethod, HttpReqParams},
  };

  #[pkg::aux]
  impl<DRSR> KuCoinHttpPkgsAux<DRSR> {}

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

  #[derive(Debug, serde::Serialize)]
  #[serde(rename_all = "camelCase")]
  #[pkg::req_data]
  pub struct V1PlaceOrderReq<'any> {
    #[serde(flatten)]
    common: V1PlaceOrderCommon<'any>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    limit: Option<V1PlaceOrderLimit<'any>>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    market: Option<V1PlaceOrderMarket<'any>>,
  }

  #[pkg::res_data]
  pub type V1PlaceOrderRes = HttpResWrapper<V1PlaceOrder>;

  #[derive(Debug, serde::Deserialize)]
  #[serde(rename_all = "camelCase")]
  #[doc = _generic_res_data_elem_doc!()]
  pub struct V1PlaceOrder {
    /// Returned unique id.
    pub order_id: ArrayString<32>,
  }

  /// Common parameters used by limit and market orders
  #[derive(Debug, serde::Serialize)]
  #[serde(rename_all = "camelCase")]
  pub struct V1PlaceOrderCommon<'any> {
    /// Custom order id.
    pub client_oid: &'any SyncDynDebugDisplay,
    /// Metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<&'any str>,
    /// Side.
    pub side: OrderSide,
    /// Self trade prevention.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stp: Option<OrderStp>,
    /// Pair of two assets like BTC-USDT.
    pub symbol: &'any str,
    /// Type.
    pub r#type: OrderType,
  }

  /// Limit order parameters
  #[derive(Debug, serde::Serialize)]
  #[serde(rename_all = "camelCase")]
  pub struct V1PlaceOrderLimit<'any> {
    /// Cancels after the given seconds. Requires `time_in_force` to be [OrderTimeInForce::GTT].
    pub cancel_after: Option<u32>,
    /// If order will or will not be displayed in the order book.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    /// Only a portion of the order is displayed in the order book
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iceberg: Option<bool>,
    /// Invalid when `time_in_force` is [OrderTimeInForce::IOC] or [OrderTimeInForce::FOK].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    /// Quote asset price.
    pub price: &'any SyncDynDebugDisplay,
    /// Base asset amount.
    pub size: &'any SyncDynDebugDisplay,
    /// Time in force.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<OrderTimeInForce>,
    /// The maximum visible size of an iceberg order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible_size: Option<&'any SyncDynDebugDisplay>,
  }

  /// Market order parameters. Requires one of the two optional values.
  #[derive(Debug, serde::Serialize)]
  #[serde(rename_all = "camelCase")]
  pub struct V1PlaceOrderMarket<'any> {
    /// Quote asset amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funds: Option<&'any SyncDynDebugDisplay>,
    /// Base asset amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<&'any SyncDynDebugDisplay>,
  }
}
