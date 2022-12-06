use crate::misc::{_MaxAssetAbbr, _MaxAssetName, _MaxNumberStr, _MaxPairAbbr};
use arrayvec::ArrayString;
use core::fmt::{Display, Formatter};
use lucia::misc::{GenericTime, QueryWriter};

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

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Debug)]
#[doc = _generic_res_data_elem_doc!()]
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
pub struct ResponseWrapper<T> {
  /// System code
  pub code: ArrayString<8>,
  /// Actual data
  pub data: crate::Result<T>,
}

#[cfg(feature = "serde")]
mod generic_data_response_serde {
  use crate::exchange::ku_coin::ResponseWrapper;

  impl<'de, T> serde::Deserialize<'de> for ResponseWrapper<T>
  where
    T: serde::Deserialize<'de> + 'de,
  {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<ResponseWrapper<T>, D::Error>
    where
      D: serde::de::Deserializer<'de>,
    {
      struct CustomVisitor<'de, T>(core::marker::PhantomData<&'de T>);

      impl<'de, T> serde::de::Visitor<'de> for CustomVisitor<'de, T>
      where
        T: serde::Deserialize<'de>,
      {
        type Value = ResponseWrapper<T>;

        #[inline]
        fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
          formatter.write_str("struct JsonRpcResponse")
        }

        #[inline]
        fn visit_map<V>(self, mut map: V) -> Result<ResponseWrapper<T>, V::Error>
        where
          V: serde::de::MapAccess<'de>,
        {
          let mut code = None;
          let mut data = None;
          let mut msg = None;

          while let Some(key) = map.next_key()? {
            match key {
              Field::Code => {
                if code.is_some() {
                  return Err(serde::de::Error::duplicate_field("code"));
                }
                code = Some(map.next_value()?);
              }
              Field::Data => {
                if data.is_some() {
                  return Err(serde::de::Error::duplicate_field("data"));
                }
                data = Some(map.next_value()?);
              }
              Field::Msg => {
                if msg.is_some() {
                  return Err(serde::de::Error::duplicate_field("msg"));
                }
                msg = Some(map.next_value()?);
              }
            }
          }

          Ok(ResponseWrapper {
            code: if let Some(elem) = code {
              elem
            } else {
              return Err(serde::de::Error::missing_field("code"));
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

pub(crate) fn manage_paginated_params(
  qw: QueryWriter<'_, String>,
  params: Option<[u32; 2]>,
) -> crate::Result<()> {
  if let Some(elem) = params {
    let _ = qw.write("currentPage", elem[0])?.write("pageSize", elem[1])?;
  }
  Ok(())
}

pub(crate) fn _timestamp() -> crate::Result<i64> {
  Ok(GenericTime::now()?.timestamp()?.as_millis().try_into()?)
}
