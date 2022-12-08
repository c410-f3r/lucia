use crate::exchange::ku_coin::{
  integration_tests::{cred_prod, cred_test, http_prod, http_test, place_order},
  V1BulletParams,
};
use lucia::network::transport::Transport;

_create_http_test!(cred_test(), http_test(), v1_get_account, |pkgs_aux, trans| async {
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

_create_http_test!(cred_test(), http_test(), v1_get_accounts, |pkgs_aux, trans| async {
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

_create_http_test!(cred_prod(), http_prod(), v1_bullet, |pkgs_aux, trans| async {
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

_create_http_test!(cred_prod(), http_prod(), v1_get_currencies, |pkgs_aux, trans| async {
  let _ = trans
    .send_retrieve_and_decode_contained(&mut pkgs_aux.v1_get_currencies().build(), pkgs_aux)
    .await
    .unwrap()
    .data
    .data
    .unwrap();
});

_create_http_test!(cred_test(), http_test(), v1_get_order, |pkgs_aux, trans| async {
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

_create_http_test!(cred_test(), http_test(), v1_get_orders, |pkgs_aux, trans| async {
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

_create_http_test!(cred_test(), http_test(), v1_place_order, |pkgs_aux, trans| async {
  place_order(pkgs_aux, trans).await;
});

_create_http_test!(cred_prod(), http_prod(), v2_get_currency, |pkgs_aux, trans| async {
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

_create_http_test!(cred_prod(), http_prod(), v1_get_ticker, |pkgs_aux, trans| async {
  let _ = trans
    .send_retrieve_and_decode_contained(
      &mut pkgs_aux.v1_get_ticker().params("BTC-USDT").build(),
      pkgs_aux,
    )
    .await
    .unwrap()
    .data
    .data
    .unwrap();
});

_create_http_test!(cred_prod(), http_prod(), v2_get_symbols, |pkgs_aux, trans| async {
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

_create_http_test!(cred_prod(), http_prod(), v3_get_full_order_book, |pkgs_aux, trans| async {
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
