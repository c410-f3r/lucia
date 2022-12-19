mod http;
mod ws;

use crate::exchange::ku_coin::{
  KuCoin, KuCoinCredentials, OrderSide, OrderType, V1PlaceOrderCommon, V1PlaceOrderMarket,
};
use lucia::{
  dnsn::SerdeJson,
  misc::GenericTime,
  network::{transport::Transport, HttpParams},
};

fn cred_prod() -> KuCoin {
  KuCoin::new(Some(
    KuCoinCredentials::from_env_vars("KU_COIN_PROD_KEY", "KU_COIN_PROD_PW", "KU_COIN_PROD_SECRET")
      .unwrap(),
  ))
  .unwrap()
}

fn cred_test() -> KuCoin {
  KuCoin::new(Some(
    KuCoinCredentials::from_env_vars("KU_COIN_TEST_KEY", "KU_COIN_TEST_PW", "KU_COIN_TEST_SECRET")
      .unwrap(),
  ))
  .unwrap()
}

fn http_prod() -> (SerdeJson, HttpParams) {
  (SerdeJson, HttpParams::from_url("https://api.kucoin.com").unwrap())
}

fn http_test() -> (SerdeJson, HttpParams) {
  (SerdeJson, HttpParams::from_url("https://openapi-sandbox.kucoin.com").unwrap())
}

async fn place_order<T>(
  pkgs_aux: &mut crate::misc::PkgsAux<KuCoin, SerdeJson, HttpParams>,
  trans: &mut T,
) where
  T: Send + Sync + Transport<SerdeJson, Params = HttpParams>,
{
  let _res = trans
    .send_retrieve_and_decode_contained(
      &mut pkgs_aux
        .v1_place_order()
        .data(
          V1PlaceOrderCommon {
            client_oid: &GenericTime::now().unwrap().timestamp().unwrap().as_nanos(),
            remark: None,
            side: OrderSide::Buy,
            stp: None,
            symbol: "BTC-USDT",
            r#type: OrderType::Market,
          },
          None,
          Some(V1PlaceOrderMarket { funds: Some(&"0.1"), size: None }),
        )
        .build(),
      pkgs_aux,
    )
    .await
    .unwrap()
    .data
    .data
    .unwrap();
}
