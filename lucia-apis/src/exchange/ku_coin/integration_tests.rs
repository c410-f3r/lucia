use crate::exchange::ku_coin::{
  KuCoin, KuCoinCredentials, V1BulletParams, V1PlaceOrderCommon, V1PlaceOrderMarket,
  V1PlaceOrderSide, V1PlaceOrderType,
};
use lucia::{
  dnsn::SerdeJson,
  misc::GenericTime,
  network::{transport::Transport, HttpParams},
};
//use lucia::package::PackagesAux;
use std::env;

_create_http_test!(KuCoin::new(None).unwrap(), http(), v1_bullet, |pkgs_aux, trans| async {
  let pkg = &mut pkgs_aux.v1_bullet().params(V1BulletParams::Public).build();
  let _ = trans.send_retrieve_and_decode_contained(pkg, pkgs_aux).await.unwrap();
});

_create_http_test!(KuCoin::new(None).unwrap(), http(), v1_currencies, |pkgs_aux, trans| async {
  let pkg = &mut pkgs_aux.v1_currencies().build();
  let _ = trans.send_retrieve_and_decode_contained(pkg, pkgs_aux).await.unwrap();
});

_create_http_test!(KuCoin::new(None).unwrap(), http(), v2_currencies, |pkgs_aux, trans| async {
  let pkg = &mut pkgs_aux.v2_currencies().params("BTC").build();
  let _ = trans.send_retrieve_and_decode_contained(pkg, pkgs_aux).await.unwrap();
});

_create_http_test!(KuCoin::new(None).unwrap(), http(), v1_symbols, |pkgs_aux, trans| async {
  let pkg = &mut pkgs_aux.v1_symbols().build();
  let _ = trans.send_retrieve_and_decode_contained(pkg, pkgs_aux).await.unwrap();
});

_create_http_test!(
  KuCoin::new(Some(
    KuCoinCredentials::new(
      env::var("KU_COIN_KEY").unwrap(),
      env::var("KU_COIN_PW").unwrap(),
      env::var("KU_COIN_SECRET").unwrap(),
    )
    .unwrap()
  ))
  .unwrap(),
  http(),
  private,
  |pkgs_aux, trans| async {
    let client_oid = &GenericTime::now().unwrap().timestamp().unwrap().as_nanos();
    let v1_place_order = &mut pkgs_aux
      .v1_place_order()
      .data(
        V1PlaceOrderCommon {
          client_oid,
          remark: None,
          side: V1PlaceOrderSide::Buy,
          stp: None,
          symbol: "BTC-USDT",
          ty: V1PlaceOrderType::Market,
        },
        None,
        Some(V1PlaceOrderMarket { funds: Some(&"0.1"), size: None }),
      )
      .build();
    let _ = trans.send_retrieve_and_decode_contained(v1_place_order, pkgs_aux).await.unwrap();

    #[cfg(feature = "tokio-tungstenite")]
    {
      use lucia::package::PackagesAux;

      let v1_bullet_pkg = &mut pkgs_aux.v1_bullet().params(V1BulletParams::Private).build();
      let v1_bullet_res =
        trans.send_retrieve_and_decode_contained(v1_bullet_pkg, pkgs_aux).await.unwrap().data.data;

      let PackagesAux { ref mut api, ref mut drsr, .. } = **pkgs_aux;
      let _pkgs_aux_ws = api.tokio_tungstenite(
        v1_bullet_res.instance_servers[0].endpoint.as_str(),
        drsr,
        Some(v1_bullet_res.token.as_str()),
      );
    }
  }
);

fn http() -> (SerdeJson, HttpParams) {
  (SerdeJson, HttpParams::from_url("https://openapi-sandbox.kucoin.com").unwrap())
}
