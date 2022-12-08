use crate::exchange::ku_coin::{
  integration_tests::{cred_prod, cred_test, http_prod, http_test, place_order},
  AccountBalance, KuCoin, V1BulletParams, WsReqTy, WsResWrapper,
};
use futures::StreamExt;
use lucia::{
  data_format::JsonResponse,
  dnsn::{Deserialize, SerdeJson},
  network::{transport::Transport, HttpParams},
};

macro_rules! sub_and_unsub {
  ($pkgs_aux:expr, $trans:expr, $sub:expr, $unsub:expr) => {
    let _ = $trans.send_retrieve_and_decode_contained($sub, $pkgs_aux).await.unwrap();
    unsub!($pkgs_aux, $trans, $unsub)
  };
}

macro_rules! unsub {
  ($pkgs_aux:expr, $trans:expr, $pkg:expr) => {
    let _ = $trans.send($pkg, $pkgs_aux).await.unwrap();
    loop {
      tokio::select! {
        _ = tokio::time::sleep(core::time::Duration::from_secs(1)) => { break },
        _ = $trans.next() => {},
      }
    }
  };
}

_create_http_test!(cred_prod(), http_prod(), ws_prod, |pkgs_aux, trans| async {
  let mut pair_ws = init_ws_instance(pkgs_aux, trans).await;
  let (pkgs_aux_ws, trans_ws) = pair_ws.parts_mut();

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

  let _ = trans_ws
    .send(&mut pkgs_aux_ws.account_balance().data(WsReqTy::Subscribe).unwrap().build(), pkgs_aux_ws)
    .await
    .unwrap();
  place_order(pkgs_aux, trans).await;
  let _ = JsonResponse::<WsResWrapper<AccountBalance>>::from_bytes(
    &trans_ws.next().await.unwrap().unwrap().into_data(),
    &mut pkgs_aux.drsr,
  )
  .unwrap();
  unsub!(
    pkgs_aux_ws,
    trans_ws,
    &mut pkgs_aux_ws.account_balance().data(WsReqTy::Unsubscribe).unwrap().build()
  );
});

async fn init_ws_instance<'pa, T>(
  pkgs_aux: &'pa mut crate::misc::PackagesAux<KuCoin, SerdeJson, HttpParams>,
  trans: &mut T,
) -> lucia::misc::Pair<
  crate::misc::PackagesAux<KuCoin, SerdeJson, lucia::network::WsParams>,
  lucia::network::transport::TokioTungstenite,
>
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
  KuCoin::tokio_tungstenite(
    v1_bullet_res.instance_servers[0].endpoint.as_str(),
    SerdeJson,
    Some(v1_bullet_res.token.as_str()),
  )
  .await
  .unwrap()
}
