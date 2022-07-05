use crate::{
  api::{
    blockchain::ethereum::{BlockNumber, Ethereum},
    Api,
  },
  network::Transport,
};

_create_http_test!(http(), eth_block_number, |rb, trans| async {
  let req = rb.eth_block_number();
  let _ = trans.send_retrieve_and_decode_one(&req, rb.tp_mut()).await.unwrap();
});

_create_http_test!(http(), eth_block_transaction_count_by_number, |rb, trans| async {
  let req = rb.eth_block_transaction_count_by_number(BlockNumber::Number(13162668));
  let _ = trans.send_retrieve_and_decode_one(&req, rb.tp_mut()).await.unwrap();
});

fn http() -> Ethereum {
  Ethereum::new("https://cloudflare-eth.com", None).unwrap()
}
