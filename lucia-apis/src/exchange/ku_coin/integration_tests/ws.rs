use crate::exchange::ku_coin::{
  integration_tests::{cred_prod, cred_test, http_prod, http_test, place_order},
  AccountBalance, KuCoin, KuCoinHttpPkgsAux, KuCoinWsPkgsAux, V1BulletParams, WsReqTy,
  WsResWrapper,
};
use lucia::{
  data_format::JsonResponse,
  dnsn::{Deserialize, SerdeJson},
  network::{
    transport::{TokioTungstenite, Transport},
    HttpParams, WebSocket,
  },
};

macro_rules! sub_and_unsub {
  ($pkgs_aux:expr, $trans:expr, $sub:expr, $unsub:expr) => {
    let _res = $trans.send_retrieve_and_decode_contained($sub, $pkgs_aux).await.unwrap();
    unsub!($pkgs_aux, $trans, $unsub)
  };
}

macro_rules! unsub {
  ($pkgs_aux:expr, $trans:expr, $pkg:expr) => {
    let _ = $trans.send($pkg, $pkgs_aux).await.unwrap();
    let mut buffer = Vec::new();
    loop {
      tokio::select! {
        _ = tokio::time::sleep(core::time::Duration::from_secs(1)) => { break },
        _ = $trans.receive_with_buffer(&mut buffer) => {},
      }
    }
  };
}

_create_http_test!(cred_prod(), http_prod(), ws_prod, |pkgs_aux, trans| async {
  let mut pair_ws = init_ws_instance(pkgs_aux, trans).await;
  let (pkgs_aux_ws, trans_ws) = pair_ws.parts_mut();

  let _res = trans_ws
    .send_retrieve_and_decode_contained(
      &mut pkgs_aux_ws.ping().data().unwrap().build(),
      &mut **pkgs_aux_ws,
    )
    .await
    .unwrap();

  sub_and_unsub!(
    pkgs_aux_ws,
    trans_ws,
    &mut pkgs_aux_ws.l2_market_data().data("BTC-USDT", WsReqTy::Subscribe).unwrap().build(),
    &mut pkgs_aux_ws.l2_market_data().data("BTC-USDT", WsReqTy::Unsubscribe).unwrap().build()
  );

  sub_and_unsub!(
    pkgs_aux_ws,
    trans_ws,
    &mut pkgs_aux_ws.symbol_ticker().data("BTC-USDT", WsReqTy::Subscribe).unwrap().build(),
    &mut pkgs_aux_ws.symbol_ticker().data("BTC-USDT", WsReqTy::Unsubscribe).unwrap().build()
  );
});

_create_http_test!(cred_test(), http_test(), ws_test, |pkgs_aux, trans| async {
  let mut pair_ws = init_ws_instance(pkgs_aux, trans).await;
  let (pkgs_aux_ws, trans_ws) = pair_ws.parts_mut();

  let _res = trans_ws
    .send(&mut pkgs_aux_ws.account_balance().data(WsReqTy::Subscribe).unwrap().build(), pkgs_aux_ws)
    .await
    .unwrap();
  place_order(pkgs_aux, trans).await;
  let _res = JsonResponse::<WsResWrapper<AccountBalance>>::from_bytes(
    &{
      let mut buffer = Vec::new();
      let _res = trans_ws.receive_with_buffer(&mut buffer).await.unwrap();
      buffer
    },
    &mut pkgs_aux.drsr,
  )
  .unwrap();
  unsub!(
    pkgs_aux_ws,
    trans_ws,
    &mut pkgs_aux_ws.account_balance().data(WsReqTy::Unsubscribe).unwrap().build()
  );
});

async fn init_ws_instance<T>(
  pkgs_aux: &mut KuCoinHttpPkgsAux<SerdeJson>,
  trans: &mut T,
) -> lucia::misc::Pair<KuCoinWsPkgsAux<SerdeJson>, TokioTungstenite>
where
  T: Send + Sync + Transport<SerdeJson, Params = HttpParams>,
{
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
  KuCoin::web_socket(
    v1_bullet_res.instance_servers[0].endpoint.as_str(),
    &mut pkgs_aux.byte_buffer,
    SerdeJson,
    Some(v1_bullet_res.token.as_str()),
  )
  .await
  .unwrap()
}
