use crate::health::m_media_covid_19::{
  CasesParams, CasesRes, HistoryParams, HistoryRes, HistoryStatus, VaccineRes, VaccinesParams,
};
use lucia::{
  dnsn::SerdeJson,
  misc::CommonParams,
  network::{http::ReqParams, Transport},
};

_create_http_test!(
  http(),
  cases,
  |rmw, trans| async {
    let req = rmw.cases();
    assert!(matches!(
      trans.send_retrieve_and_decode_one(rmw, &req, CasesParams::new(None, None, None)).await.unwrap(),
      CasesRes::Many(_)
    ));
    assert!(matches!(
      trans
      .send_retrieve_and_decode_one(rmw, &req, CasesParams::new(Some("pt"), None, None))
      .await
      .unwrap(),
      CasesRes::One(_)
    ));
  },
  #[ignore]
);

_create_http_test!(http(), history, |rmw, trans| async {
  let req = rmw.history();
  assert!(matches!(
    trans
      .send_retrieve_and_decode_one(
        rmw,
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
        rmw,
        &req,
        HistoryParams::new(HistoryStatus::Confirmed, Some("pt"), None, None)
      )
      .await
      .unwrap(),
    HistoryRes::One(_)
  ));
});

_create_http_test!(http(), vaccines, |rmw, trans| async {
  let req = rmw.vaccines();
  assert!(matches!(
    trans
      .send_retrieve_and_decode_one(rmw, &req, VaccinesParams::new(None, None, None))
      .await
      .unwrap(),
    VaccineRes::Many(_)
  ));
  assert!(matches!(
    trans
      .send_retrieve_and_decode_one(rmw, &req, VaccinesParams::new(Some("pt"), None, None))
      .await
      .unwrap(),
    VaccineRes::One(_)
  ));
});

fn http() -> (CommonParams<ReqParams, ()>, SerdeJson) {
  (
    CommonParams::new(ReqParams::from_origin("https://covid-api.mmediagroup.fr").unwrap(), ()),
    SerdeJson::default(),
  )
}
