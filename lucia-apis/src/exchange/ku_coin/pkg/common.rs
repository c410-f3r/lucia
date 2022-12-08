use crate::misc::{ConcatArrayStr, _MaxAssetAbbr, _MaxAssetName, _MaxNumberStr, _MaxPairAbbr};
use arrayvec::ArrayString;
use core::fmt::{Display, Formatter};
use lucia::misc::QueryWriter;

pub(crate) type Chain = ArrayString<20>;
pub(crate) type KuCoinId = ArrayString<28>;

/// Buy or sell
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[derive(Clone, Copy, Debug)]
pub enum OrderSide {
  /// Buy
  Buy,
  /// Sell
  Sell,
}

impl Display for OrderSide {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    f.write_str(match *self {
      Self::Buy => "buy",
      Self::Sell => "sell",
    })
  }
}

/// Stopping criteria to prevent possible losses.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[derive(Clone, Copy, Debug)]
pub enum OrderStop {
  /// A stop-entry order to buy is an order at a price above the prevailing market price, and a
  /// stop-entry order to sell is an order at a price below the prevailing market price.
  Entry,
  //// A stop-loss order is a market order that helps manage risk by closing your position once the
  /// instrument​​/asset reaches a certain price.
  Loss,
}

/// Self trade prevention
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "UPPERCASE"))]
#[derive(Clone, Copy, Debug)]
pub enum OrderStp {
  /// Cancel both
  CB,
  /// Cancel newest
  CN,
  /// Cancel oldest
  CO,
  /// Decrease and Cancel
  DC,
}

/// Guarantees about the lifetime of an order.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "UPPERCASE"))]
#[derive(Clone, Copy, Debug)]
pub enum OrderTimeInForce {
  /// Fill or kill
  FOK,
  /// Good till canceled.
  GTC,
  /// Good till time.
  GTT,
  /// Immediate or cancel.
  IOC,
}

/// KuCoin has three different types of accounts.
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[derive(Clone, Copy, Debug)]
pub enum V1AccountTy {
  /// Storage, withdrawal, and deposit of funds.
  Main,
  /// Borrow assets and leverage transactions.
  Margin,
  /// Trading of orders in the spot market.
  Trade,
}

impl Display for V1AccountTy {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    f.write_str(match *self {
      Self::Main => "main",
      Self::Margin => "margin",
      Self::Trade => "trade",
    })
  }
}

/// Limit or market.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[derive(Debug)]
pub enum OrderType {
  /// Executed at the specified price.
  Limit,
  /// Immediately executed with the current market price.
  Market,
}

/// If a web socket request is asking to subscribe or unsubscribe.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[derive(Clone, Copy, Debug)]
pub enum WsReqTy {
  /// Subscribe
  Subscribe,
  /// Unsubscribe.
  Unsubscribe,
}

/// All responses are wrapped to provide additional metadata.
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[derive(Debug)]
pub enum WsResWrapperSubject {
  /// Account balance
  #[cfg_attr(feature = "serde", serde(rename = "account.balance"))]
  AccountBalance,
  /// L2 market data
  #[cfg_attr(feature = "serde", serde(rename = "trade.l2update"))]
  TradeL2Update,
  /// For example, tickers.
  Other(String),
}

/// Value depending on the issued request type.
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[derive(Clone, Copy, Debug)]
pub enum WsResWrapperTy {
  /// Error
  Error,
  /// Message,
  Message,
  /// Subscribe.
  Subscribe,
  /// Welcome
  Welcome,
}

/// Account has different types of balances.
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Debug)]
pub struct V1Account {
  /// Funds available to withdraw or trade.
  pub available: _MaxNumberStr,
  /// Asset identifier
  pub currency: Option<_MaxAssetName>,
  /// Frozen amount.
  pub holds: _MaxNumberStr,
  /// Account type
  pub r#type: Option<V1AccountTy>,
}

/// Order
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct V1Order {
  /// Cancels after the given seconds. Requires `time_in_force` to be [OrderTimeInForce::GTT].
  pub cancel_after: i64,
  /// Order cancellation transaction record
  pub cancel_exist: bool,
  /// Order source.
  pub channel: ArrayString<8>,
  /// User-provided ID at creation time.
  pub client_oid: ArrayString<20>,
  /// Creation timestamp.
  pub created_at: i64,
  /// Dealt amount of quote asset.
  pub deal_funds: _MaxNumberStr,
  /// Dealt amount of base asset.
  pub deal_size: _MaxNumberStr,
  /// Fee amount
  pub fee: _MaxNumberStr,
  /// Charged fees in the given asset.
  pub fee_currency: _MaxAssetAbbr,
  /// Quote asset amount.
  pub funds: _MaxNumberStr,
  /// If order will or will not be displayed in the order book.
  pub hidden: bool,
  /// Only a portion of the order is displayed in the order book
  pub iceberg: bool,
  /// Unique identifier of an order.
  pub id: KuCoinId,
  /// If true, the order is active, if false, the order is filled or cancelled
  pub is_active: Option<bool>,
  /// Invalid when `time_in_force` is [OrderTimeInForce::IOC] or [OrderTimeInForce::FOK].
  pub post_only: bool,
  /// Quote asset price.
  pub price: _MaxNumberStr,
  /// Side.
  pub side: OrderSide,
  /// Base asset amount.
  pub size: _MaxNumberStr,
  /// Stop type, include entry and loss.
  #[cfg_attr(
    feature = "serde",
    serde(deserialize_with = "crate::misc::_deserialize_opt_considering_empty_str")
  )]
  pub stop: Option<OrderStop>,
  /// Stop price
  pub stop_price: _MaxNumberStr,
  /// If stop type is triggered.
  pub stop_triggered: bool,
  /// Self trade prevention.
  #[cfg_attr(
    feature = "serde",
    serde(deserialize_with = "crate::misc::_deserialize_opt_considering_empty_str")
  )]
  pub stp: Option<OrderStp>,
  /// Pair of two assets like BTC-USDT.
  pub symbol: _MaxPairAbbr,
  /// Time in force.
  pub time_in_force: OrderTimeInForce,
  /// Type.
  pub r#type: OrderType,
  /// The maximum visible size of an iceberg order.
  pub visible_size: _MaxNumberStr,
}

/// Best and last values of the level 1 market data.
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct V1Ticker {
  /// Sequence
  #[cfg_attr(feature = "serde", serde(deserialize_with = "crate::misc::_deserialize_from_str"))]
  pub sequence: u64,
  /// Best ask price
  pub best_ask: _MaxNumberStr,
  /// Last traded size
  pub size: _MaxNumberStr,
  /// Last traded price
  pub price: _MaxNumberStr,
  /// Best bid size
  pub best_bid_size: _MaxNumberStr,
  /// Best bid price
  pub best_bid: _MaxNumberStr,
  /// Best ask size
  pub best_ask_size: _MaxNumberStr,
  /// timestamp
  pub time: u64,
}

/// For endpoints that return very large amounts of items.
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct PaginatedResponse<T> {
  /// Current page
  pub current_page: u32,
  /// Current size
  pub page_size: u32,
  /// Total number
  pub total_num: u32,
  /// Total page
  pub total_page: u32,
  /// Items
  pub items: Vec<T>,
}

/// All responses are wrapped to provide additional metadata.
#[derive(Debug)]
pub struct HttpResWrapper<T> {
  /// System code
  pub code: u32,
  /// Actual data
  pub data: crate::Result<T>,
}

#[cfg(feature = "serde")]
mod generic_data_response_serde {
  use crate::exchange::ku_coin::HttpResWrapper;
  use serde::{de, de::Error, Deserialize};

  impl<'de, T> Deserialize<'de> for HttpResWrapper<T>
  where
    T: Deserialize<'de> + 'de,
  {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<HttpResWrapper<T>, D::Error>
    where
      D: de::Deserializer<'de>,
    {
      struct CustomVisitor<'de, T>(core::marker::PhantomData<&'de T>);

      impl<'de, T> de::Visitor<'de> for CustomVisitor<'de, T>
      where
        T: Deserialize<'de>,
      {
        type Value = HttpResWrapper<T>;

        #[inline]
        fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
          formatter.write_str("struct JsonRpcResponse")
        }

        #[inline]
        fn visit_map<V>(self, mut map: V) -> Result<HttpResWrapper<T>, V::Error>
        where
          V: de::MapAccess<'de>,
        {
          let mut code = None;
          let mut data = None;
          let mut msg = None;

          while let Some(key) = map.next_key()? {
            match key {
              Field::Code => {
                if code.is_some() {
                  return Err(de::Error::duplicate_field("code"));
                }
                code = Some(map.next_value::<&str>()?.parse().map_err(|err| Error::custom(err))?);
              }
              Field::Data => {
                if data.is_some() {
                  return Err(de::Error::duplicate_field("data"));
                }
                data = Some(map.next_value()?);
              }
              Field::Msg => {
                if msg.is_some() {
                  return Err(de::Error::duplicate_field("msg"));
                }
                msg = Some(map.next_value()?);
              }
            }
          }

          Ok(HttpResWrapper {
            code: if let Some(elem) = code {
              elem
            } else {
              return Err(de::Error::missing_field("code"));
            },
            data: if let Some(elem) = data {
              Ok(elem)
            } else {
              Err(crate::Error::KuCoinUnsuccessfulRequest(msg.unwrap_or_default()))
            },
          })
        }
      }

      const FIELDS: &[&str] = &["error", "result"];
      deserializer.deserialize_struct(
        "JsonRpcResponse",
        FIELDS,
        CustomVisitor(core::marker::PhantomData),
      )
    }
  }

  #[derive(serde::Deserialize)]
  #[serde(field_identifier, rename_all = "lowercase")]
  enum Field {
    Code,
    Data,
    Msg,
  }
}

/// All WebSocket requests must have a pre-defined set of fields.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct WsReq<'any> {
  pub(crate) id: u64,
  pub(crate) private_channel: bool,
  pub(crate) response: bool,
  pub(crate) topic: ConcatArrayStr<'any, 2>,
  pub(crate) r#type: WsReqTy,
}

/// All responses are wrapped to provide additional metadata.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct WsResWrapper<D> {
  /// Type
  pub r#type: WsResWrapperTy,
  /// Seme as the "topid" request parameter.
  pub topic: String,
  /// Subject
  pub subject: WsResWrapperSubject,
  /// Depends on the request
  pub data: D,
}

pub(crate) fn manage_paginated_params(
  qw: QueryWriter<'_, String>,
  params: Option<[u32; 2]>,
) -> crate::Result<()> {
  if let Some(elem) = params {
    let _ = qw.write("currentPage", elem[0])?.write("pageSize", elem[1])?;
  }
  Ok(())
}
