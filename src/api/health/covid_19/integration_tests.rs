#![cfg(all(test, feature = "_integration-tests"))]

use crate::{
  api::health::covid_19::{
    CasesParams, CasesRes, HistoryParams, HistoryRes, HistoryStatus, VaccineRes, VaccinesParams,
  },
  network::{HttpParams, Transport},
};

_create_http_test!(http(), cases, |rm, trans| async {
  let req = rm.cases();
  assert!(matches!(
    trans.send_retrieve_and_decode_one(rm, &req, CasesParams::new(None, None, None)).await.unwrap(),
    CasesRes::Many(_)
  ));
  assert!(matches!(
    trans
      .send_retrieve_and_decode_one(rm, &req, CasesParams::new(Some("pt"), None, None))
      .await
      .unwrap(),
    CasesRes::One(_)
  ));
});

_create_http_test!(http(), history, |rm, trans| async {
  let req = rm.history();
  assert!(matches!(
    trans
      .send_retrieve_and_decode_one(
        rm,
        &req,
        HistoryParams::new(HistoryStatus::Confirmed, None, None, None)
      )
      .await
      .unwrap(),
    HistoryRes::Many(_)
  ));
  assert!(matches!(
    trans
      .send_retrieve_and_decode_one(
        rm,
        &req,
        HistoryParams::new(HistoryStatus::Confirmed, Some("pt"), None, None)
      )
      .await
      .unwrap(),
    HistoryRes::One(_)
  ));
});

_create_http_test!(http(), vaccines, |rm, trans| async {
  let req = rm.vaccines();
  assert!(matches!(
    trans
      .send_retrieve_and_decode_one(rm, &req, VaccinesParams::new(None, None, None))
      .await
      .unwrap(),
    VaccineRes::Many(_)
  ));
  assert!(matches!(
    trans
      .send_retrieve_and_decode_one(rm, &req, VaccinesParams::new(Some("pt"), None, None))
      .await
      .unwrap(),
    VaccineRes::One(_)
  ));
});

fn http() -> HttpParams {
  HttpParams::from_origin("https://covid-api.mmediagroup.fr").unwrap()
}
