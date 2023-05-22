use crate::{
  blockchain::aptos::Aptos,
  misc::{init_test_cfg, PkgsAux},
};
use lucia::{
  dnsn::SerdeJson,
  network::{transport::Transport, HttpParams},
};

create_http_test!(Aptos::new(None), http(), check_basic_node_health, |pkgs_aux, trans| async {
  let _res = trans
    .send_retrieve_and_decode_contained(
      &mut pkgs_aux.check_basic_node_health().params(None).build(),
      pkgs_aux,
    )
    .await
    .unwrap()
    .data;
});

fn http() -> (SerdeJson, HttpParams) {
  (SerdeJson, HttpParams::from_url("https://fullnode.devnet.aptoslabs.com/v1").unwrap())
}
