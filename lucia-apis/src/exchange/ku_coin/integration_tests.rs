use crate::exchange::ku_coin::{
  KuCoin, KuCoinCredentials, OrderSide, OrderType, V1BulletParams, V1PlaceOrderCommon,
  V1PlaceOrderMarket,
};
use lucia::{
  dnsn::SerdeJson,
  misc::GenericTime,
  network::{transport::Transport, HttpParams},
};

_create_http_test!(ku_coin_priv(), http_test(), v1_get_account, |pkgs_aux, trans| async {
  let _ = trans
    .send_retrieve_and_decode_contained(
      &mut pkgs_aux.v1_get_account().params("63876a3029c692000127a3ec").build(),
      pkgs_aux,
    )
    .await
    .unwrap()
    .data
    .data
    .unwrap();
});

_create_http_test!(ku_coin_priv(), http_test(), v1_get_accounts, |pkgs_aux, trans| async {
  let _ = trans
    .send_retrieve_and_decode_contained(
      &mut pkgs_aux.v1_get_accounts().params(None, None).build(),
      pkgs_aux,
    )
    .await
    .unwrap()
    .data
    .data
    .unwrap();
});

_create_http_test!(ku_coin_pub(), http_test(), v1_bullet, |pkgs_aux, trans| async {
  let _ = trans
    .send_retrieve_and_decode_contained(
      &mut pkgs_aux.v1_bullet().params(V1BulletParams::Public).build(),
      pkgs_aux,
    )
    .await
    .unwrap()
    .data
    .data
    .unwrap();
});

_create_http_test!(ku_coin_pub(), http_test(), v1_get_currencies, |pkgs_aux, trans| async {
  let _ = trans
    .send_retrieve_and_decode_contained(&mut pkgs_aux.v1_get_currencies().build(), pkgs_aux)
    .await
    .unwrap()
    .data
    .data
    .unwrap();
});

_create_http_test!(ku_coin_priv(), http_test(), v1_get_order, |pkgs_aux, trans| async {
  let _ = trans
    .send_retrieve_and_decode_contained(
      &mut pkgs_aux.v1_get_order().params("638bc34a0091a60001d8d35a").build(),
      pkgs_aux,
    )
    .await
    .unwrap()
    .data
    .data
    .unwrap();
});

_create_http_test!(ku_coin_priv(), http_test(), v1_get_orders, |pkgs_aux, trans| async {
  let _ = trans
    .send_retrieve_and_decode_contained(
      &mut pkgs_aux.v1_get_orders().params(None, None, None, None).build(),
      pkgs_aux,
    )
    .await
    .unwrap()
    .data
    .data
    .unwrap();
});

_create_http_test!(ku_coin_priv(), http_test(), v1_place_order, |pkgs_aux, trans| async {
  let _ = trans
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

  #[cfg(feature = "tokio-tungstenite")]
  {
    use lucia::package::PackagesAux;

    let v1_bullet_res = trans
      .send_retrieve_and_decode_contained(
        &mut pkgs_aux.v1_bullet().params(V1BulletParams::Private).build(),
        pkgs_aux,
      )
      .await
      .unwrap()
      .data
      .data
      .unwrap();

    let PackagesAux { ref mut api, ref mut drsr, .. } = **pkgs_aux;
    let _pkgs_aux_ws = api.tokio_tungstenite(
      v1_bullet_res.instance_servers[0].endpoint.as_str(),
      drsr,
      Some(v1_bullet_res.token.as_str()),
    );
  }
});

_create_http_test!(ku_coin_pub(), http_test(), v2_currencies, |pkgs_aux, trans| async {
  let _ = trans
    .send_retrieve_and_decode_contained(
      &mut pkgs_aux.v2_get_currency().params("BTC").build(),
      pkgs_aux,
    )
    .await
    .unwrap()
    .data
    .data
    .unwrap();
});

_create_http_test!(ku_coin_pub(), http_prod(), v2_symbols, |pkgs_aux, trans| async {
  let _ = trans
    .send_retrieve_and_decode_contained(
      &mut pkgs_aux.v2_get_symbols().params(None).build(),
      pkgs_aux,
    )
    .await
    .unwrap()
    .data
    .data
    .unwrap();
});

_create_http_test!(ku_coin_priv(), http_test(), v3_get_full_order_book, |pkgs_aux, trans| async {
  let _ = trans
    .send_retrieve_and_decode_contained(
      &mut pkgs_aux.v3_get_full_order_book().params("BTC-USDT").build(),
      pkgs_aux,
    )
    .await
    .unwrap()
    .data
    .data
    .unwrap();
});

fn http_prod() -> (SerdeJson, HttpParams) {
  (SerdeJson, HttpParams::from_url("https://api.kucoin.com").unwrap())
}

fn http_test() -> (SerdeJson, HttpParams) {
  (SerdeJson, HttpParams::from_url("https://openapi-sandbox.kucoin.com").unwrap())
}

fn ku_coin_priv() -> KuCoin {
  KuCoin::new(Some(KuCoinCredentials::from_default_env_vars().unwrap())).unwrap()
}

fn ku_coin_pub() -> KuCoin {
  KuCoin::new(None).unwrap()
}
