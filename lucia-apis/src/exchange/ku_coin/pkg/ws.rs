macro_rules! create {
  (
    $pkg:ident,
    $fn:ident,
    ($($param_ident:ident: $param_ty:ty )?),
    $private_channel:expr,
    $array:expr,
    $req_ident:ident => $req_ty:ty,
    $res_ident:ident => $res_ty:ty,
    { $($additional_item:item)* }
  ) => {
    pub use $pkg::*;
    #[lucia_macros::pkg(api(KuCoin), data_format(json), error(crate::Error), transport(ws))]
    pub(crate) mod $pkg {
      use crate::{
        exchange::ku_coin::{KuCoin, KuCoinWsPkgsAux, WsReq, WsReqTy, WsResWrapper},
        misc::concat_array_str::ConcatArrayStr,
      };
      use lucia::{misc::GenericTime, network::WsReqParamsTy};

      #[pkg::aux]
      impl<DRSR> KuCoinWsPkgsAux<DRSR> {
        #[pkg::aux_data]
        fn $fn<'any>(
          &mut self,
          $( $param_ident: $param_ty, )?
          r#type: WsReqTy,
        ) -> crate::Result<$req_ident<'any>> {
          self.ext_req_params.ty = WsReqParamsTy::String;
          Ok(WsReq {
            id: GenericTime::now()?.timestamp()?.as_nanos().try_into()?,
            private_channel: $private_channel,
            response: false,
            topic: ConcatArrayStr($array),
            r#type,
          })
        }
      }

      #[pkg::req_data]
      pub type $req_ident<'any> = $req_ty;

      #[pkg::res_data]
      pub type $res_ident = $res_ty;

      $($additional_item)*
    }
  };
}

create!(
  account_balance,
  account_balance_data,
  (),
  true,
  ["/account/balance", ""],
  AccountBalanceReq => WsReq<'any>,
  AccountBalanceRes => WsResWrapper<AccountBalance>,
  {
    use crate::exchange::ku_coin::KuCoinId;
    use crate::misc::{_MaxAssetAbbr, _MaxNumberStr};

    /// Account balance modification returned by a WebSocket response.
    #[derive(Debug, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
        pub struct AccountBalance {
      /// KuCoin account id.
      pub account_id: KuCoinId,
      /// The change of available balance
      pub available_change: _MaxNumberStr,
      /// Available balance
      pub available: _MaxNumberStr,
      /// Asset
      pub currency: _MaxAssetAbbr,
      /// The change of hold balance
      pub hold_change: _MaxNumberStr,
      /// Hold amount
      pub hold: _MaxNumberStr,
      /// Total balance
      pub total: _MaxNumberStr,
    }
  }
);

create!(
  l2_market_data,
  l2_market_data_data,
  (symbols: &'any str),
  false,
  ["/market/level2:", symbols],
  L2MarketDataReq => WsReq<'any>,
  L2MarketDataRes => WsResWrapper<L2MarketData>,
  {
    use crate::misc::{_MaxNumberStr, _MaxPairAbbr};

    /// Response of a WebSocket level 2 market data.
    #[derive(Debug, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct L2MarketData {
      /// Changes
      pub changes: L2MarketDataChanges,
      /// KuCoin-specified ending sequence.
      pub sequence_end: u64,
      /// KuCoin-specified starting sequence.
      pub sequence_start: u64,
      /// Pair of two assets
      pub symbol: _MaxPairAbbr,
    }

    /// Both asks and bids changes
    #[derive(Debug, serde::Deserialize)]
      pub struct L2MarketDataChanges {
      /// Selling offers of base asset.
      pub asks: Vec<L2MarketDataChangesValues>,
      /// Buying offers of base asset.
      pub bids: Vec<L2MarketDataChangesValues>,
    }

    /// Asks or bids changes
    #[derive(Debug, serde::Deserialize)]
    pub struct L2MarketDataChangesValues(
      /// Price
      pub _MaxNumberStr,
      /// Size
      pub _MaxNumberStr,
      /// Sequence
      #[serde(deserialize_with = "crate::misc::_deserialize_from_str")]
      pub u64,
    );
  }
);

create!(
  symbol_ticker,
  symbol_ticker_data,
  (symbol: &'any str),
  false,
  ["/market/ticker:", symbol],
  SymbolTickerReq => WsReq<'any>,
  SymbolTickerRes => WsResWrapper<crate::exchange::ku_coin::V1Ticker>,
  {}
);

pub use ping::*;
#[lucia_macros::pkg(api(KuCoin), data_format(json), error(crate::Error), transport(ws))]
pub(crate) mod ping {
  use crate::exchange::ku_coin::{KuCoin, KuCoinWsPkgsAux};
  use arrayvec::ArrayString;
  use lucia::{misc::GenericTime, network::WsReqParamsTy};

  #[pkg::aux]
  impl<DRSR> KuCoinWsPkgsAux<DRSR> {
    #[pkg::aux_data]
    fn ping_data(&mut self) -> crate::Result<PingReq> {
      self.ext_req_params.ty = WsReqParamsTy::String;
      Ok(PingReq { id: GenericTime::now()?.timestamp()?.as_nanos().try_into()?, r#type: "ping" })
    }
  }

  #[derive(Debug, serde::Serialize)]
  #[pkg::req_data]
  pub struct PingReq {
    id: u64,
    r#type: &'static str,
  }

  #[derive(Debug, serde::Deserialize)]
  #[pkg::res_data]
  pub struct PingRes {
    /// KuCoin ID
    #[serde(deserialize_with = "crate::misc::_deserialize_from_str")]
    pub id: u64,
    /// Pong string
    pub r#type: ArrayString<4>,
  }
}
