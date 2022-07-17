#![cfg(all(test, feature = "_integration-tests"))]

use crate::{
  api::blockchain::ethereum::BlockNumber,
  network::{HttpParams, Transport},
};

_create_http_test!(http(), eth_block_number, |rm, trans| async {
  let req = rm.eth_block_number();
  let _ = trans.send_retrieve_and_decode_one(rm, &req, ()).await.unwrap();
});

_create_http_test!(http(), eth_block_transaction_count_by_number, |rm, trans| async {
  let req = rm.eth_block_transaction_count_by_number(BlockNumber::Number(13162668));
  let _ = trans.send_retrieve_and_decode_one(rm, &req, ()).await.unwrap();
});

_create_http_test!(http(), eth_get_balance, |rm, trans| async {
  let req =
    rm.eth_get_balance("0xd6216fc19db775df9774a6e33526131da7d19a2c", Some(BlockNumber::Latest));
  let _ = trans.send_retrieve_and_decode_one(rm, &req, ()).await.unwrap();
});

fn http() -> HttpParams {
  HttpParams::from_origin("https://cloudflare-eth.com").unwrap()
}
