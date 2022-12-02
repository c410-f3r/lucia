#[lucia_macros::pkg(api(KuCoin), data_format(json), error(crate::Error), transport(http))]
pub(crate) mod pkg {
  use crate::{
    exchange::ku_coin::{GenericDataResponse, KuCoin, KuCoinHttpPackagesAux},
    misc::into_rslt,
  };
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
  pub struct V1PlaceOrderReqData<'any> {
    #[cfg_attr(feature = "serde", serde(flatten))]
    common: V1PlaceOrderCommon<'any>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    limit: Option<V1PlaceOrderLimit<'any>>,
    #[cfg_attr(feature = "serde", serde(flatten))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    market: Option<V1PlaceOrderMarket<'any>>,
  }

  #[pkg::res_data]
  pub type V1PlaceOrderResData = GenericDataResponse<V1PlaceOrderElemResData>;

  #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
  #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  pub struct V1PlaceOrderElemResData {}

  /// Buy or sell
  #[derive(Debug)]
  pub enum V1PlaceOrderSide {
    /// Buy
    Buy,
    /// Sell
    Sell,
  }

  #[cfg(feature = "serde")]
  impl serde::Serialize for V1PlaceOrderSide {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: serde::Serializer,
    {
      serializer.serialize_str(match *self {
        Self::Buy => "buy",
        Self::Sell => "sell",
      })
    }
  }

  /// Self trade prevention
  #[derive(Debug)]
  pub enum V1PlaceOrderStp {
    /// Cancel both
    CB,
    /// Cancel newest
    CN,
    /// Cancel oldest
    CO,
    /// Decrease and Cancel
    DC,
  }

  #[cfg(feature = "serde")]
  impl serde::Serialize for V1PlaceOrderStp {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: serde::Serializer,
    {
      serializer.serialize_str(match *self {
        Self::CB => "CB",
        Self::CN => "CN",
        Self::CO => "CO",
        Self::DC => "DC",
      })
    }
  }

  /// Limit or market.
  #[derive(Debug)]
  pub enum V1PlaceOrderType {
    /// Executed at the specified price.
    Limit,
    /// Immediately executed with the current market price.
    Market,
  }

  #[cfg(feature = "serde")]
  impl serde::Serialize for V1PlaceOrderType {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: serde::Serializer,
    {
      serializer.serialize_str(match *self {
        Self::Limit => "limit",
        Self::Market => "market",
      })
    }
  }

  /// Guarantees about the lifetime of an order.
  #[derive(Debug)]
  pub enum V1PlaceTimeInForce {
    /// Fill or kill
    FOK,
    /// Good till canceled.
    GTC,
    /// Good till time.
    GTT,
    /// Immediate or cancel.
    IOC,
  }

  #[cfg(feature = "serde")]
  impl serde::Serialize for V1PlaceTimeInForce {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: serde::Serializer,
    {
      serializer.serialize_str(match *self {
        Self::FOK => "FOK",
        Self::GTC => "GTC",
        Self::GTT => "GTT",
        Self::IOC => "IOC",
      })
    }
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
    pub side: V1PlaceOrderSide,
    /// Self trade prevention.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub stp: Option<V1PlaceOrderStp>,
    /// Pair of two assets like BTC-USDT.
    pub symbol: &'any str,
    /// Type.
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub ty: V1PlaceOrderType,
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
    time_in_force: Option<V1PlaceTimeInForce>,
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
