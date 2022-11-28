use crate::exchange::ku_coin::KuCoin;
use lucia::{
  dnsn::SerdeJson,
  network::{transport::Transport, HttpParams},
};

_create_http_test!(KuCoin, http(), v1_currencies, |pkgs_aux, trans| async {
  let pkg = &mut pkgs_aux.v1_currencies().build();
  let _ = trans.send_retrieve_and_decode_contained(pkg, pkgs_aux).await.unwrap();
});

_create_http_test!(KuCoin, http(), v2_currencies, |pkgs_aux, trans| async {
  let pkg = &mut pkgs_aux.v2_currencies().params("BTC").build();
  let _ = trans.send_retrieve_and_decode_contained(pkg, pkgs_aux).await.unwrap();
});

_create_http_test!(KuCoin, http(), v1_symbols, |pkgs_aux, trans| async {
  let pkg = &mut pkgs_aux.v1_symbols().build();
  let _ = trans.send_retrieve_and_decode_contained(pkg, pkgs_aux).await.unwrap();
});

fn http() -> (SerdeJson, HttpParams) {
  (SerdeJson, HttpParams::from_url("https://openapi-sandbox.kucoin.com").unwrap())
}
