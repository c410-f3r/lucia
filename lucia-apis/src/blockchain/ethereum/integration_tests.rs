use crate::{
  blockchain::ethereum::{BlockNumber, Ethereum},
  misc::{init_test_cfg, PkgsAux},
};
use lucia::{
  dnsn::SerdeJson,
  network::{transport::Transport, HttpParams},
};

create_http_test!(Ethereum, http(), eth_block_number, |pkgs_aux, trans| async {
  let _res = trans
    .send_retrieve_and_decode_contained(&mut pkgs_aux.eth_block_number().build(), pkgs_aux)
    .await
    .unwrap()
    .result
    .unwrap();
});

create_http_test!(
  Ethereum,
  http(),
  eth_block_transaction_count_by_number,
  |pkgs_aux, trans| async {
    let _res = trans
      .send_retrieve_and_decode_contained(
        &mut pkgs_aux
          .eth_block_transaction_count_by_number()
          .data([&BlockNumber::Number(15228994)])
          .build(),
        pkgs_aux,
      )
      .await
      .unwrap()
      .result
      .unwrap();
  }
);

create_http_test!(Ethereum, http(), eth_get_balance, |pkgs_aux, trans| async {
  let _res = trans
    .send_retrieve_and_decode_contained(
      &mut pkgs_aux
        .eth_get_balance()
        .data("0xd6216fc19db775df9774a6e33526131da7d19a2c", &BlockNumber::Latest)
        .build(),
      pkgs_aux,
    )
    .await
    .unwrap()
    .result
    .unwrap();
});

fn http() -> (SerdeJson, HttpParams) {
  (SerdeJson, HttpParams::from_url("https://cloudflare-eth.com").unwrap())
}
