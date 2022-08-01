use crate::{
  api::exchange::ku_coin::{
    V1BulletPublicParams, V1CurrenciesParams, V1SymbolsParams, V2CurrenciesParams,
  },
  dnsn::SerdeJson,
  network::{http::ReqParams, Transport},
  CommonParams,
};

_create_http_test!(http(), v1_bullet_public, |rm, trans| async {
  let req = rm.v1_bullet_public();
  let _ = trans.send_retrieve_and_decode_one(rm, &req, V1BulletPublicParams::new()).await.unwrap();
});

_create_http_test!(http(), v1_currencies, |rm, trans| async {
  let req = rm.v1_currencies();
  let _ = trans.send_retrieve_and_decode_one(rm, &req, V1CurrenciesParams::new()).await.unwrap();
});

_create_http_test!(http(), v2_currencies, |rm, trans| async {
  let req = rm.v2_currencies();
  let _ =
    trans.send_retrieve_and_decode_one(rm, &req, V2CurrenciesParams::new("BTC")).await.unwrap();
});

_create_http_test!(http(), v1_symbols, |rm, trans| async {
  let req = rm.v1_symbols();
  let _ = trans.send_retrieve_and_decode_one(rm, &req, V1SymbolsParams::new()).await.unwrap();
});

fn http() -> (CommonParams<ReqParams, ()>, SerdeJson) {
  (
    CommonParams::new(ReqParams::from_origin("https://openapi-sandbox.kucoin.com").unwrap(), ()),
    SerdeJson::default(),
  )
}
