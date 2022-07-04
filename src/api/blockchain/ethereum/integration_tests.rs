use crate::{
  api::blockchain::ethereum::{BlockNumber, Ethereum},
  network::Transport,
  Api,
};

_create_http_test!(api(), eth_block_number, |rb, trans| async {
  let req = rb.eth_block_number();
  let _ = trans.send_retrieve_and_decode_one(&req, rb.tp_mut()).await.unwrap();
});

_create_http_test!(api(), eth_block_transaction_count_by_number, |rb, trans| async {
  let req = rb.eth_block_transaction_count_by_number(BlockNumber::Number(13162668));
  let _ = trans.send_retrieve_and_decode_one(&req, rb.tp_mut()).await.unwrap();
});

fn api() -> Ethereum {
  Ethereum::from_origin("https://cloudflare-eth.com").unwrap()
}
