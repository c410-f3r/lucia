use crate::{api::exchange::ku_coin::KuCoin, network::Transport, Api};

_create_http_test!(api(), v1_bullet_public, |rb, trans| async {
  let req = rb.v1_bullet_public();
  let _ = trans.send_retrieve_and_decode_one(&req, rb.tp_mut()).await.unwrap();
});

_create_http_test!(api(), v1_currencies, |rb, trans| async {
  let req = rb.v1_currencies();
  let _ = trans.send_retrieve_and_decode_one(&req, rb.tp_mut()).await.unwrap();
});

_create_http_test!(api(), v2_currencies, |rb, trans| async {
  let req = rb.v2_currencies("BTC").unwrap();
  let _ = trans.send_retrieve_and_decode_one(&req, rb.tp_mut()).await.unwrap();
});

_create_http_test!(api(), v1_symbols, |rb, trans| async {
  let req = rb.v1_symbols();
  let _ = trans.send_retrieve_and_decode_one(&req, rb.tp_mut()).await.unwrap();
});

fn api() -> KuCoin {
  KuCoin::from_origin("https://openapi-sandbox.kucoin.com").unwrap()
}
