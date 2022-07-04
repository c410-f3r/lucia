use crate::{
  api::health::covid_19::{
    endpoint::{CasesRes, HistoryRes, HistoryStatus, VaccineRes},
    Covid19,
  },
  network::Transport,
  Api,
};

_create_http_test!(api(), cases, |rb, trans| async {
  let req = rb.cases(None, None, None).unwrap();
  assert!(matches!(
    trans.send_retrieve_and_decode_one(&req, rb.tp_mut()).await.unwrap(),
    CasesRes::Many(_)
  ));
  let req = rb.cases(Some("pt"), None, None).unwrap();
  assert!(matches!(
    trans.send_retrieve_and_decode_one(&req, rb.tp_mut()).await.unwrap(),
    CasesRes::One(_)
  ));
});

_create_http_test!(api(), history, |rb, trans| async {
  let req = rb.history(HistoryStatus::Confirmed, None, None, None).unwrap();
  assert!(matches!(
    trans.send_retrieve_and_decode_one(&req, rb.tp_mut()).await.unwrap(),
    HistoryRes::Many(_)
  ));
  let req = rb.history(HistoryStatus::Confirmed, Some("pt"), None, None).unwrap();
  assert!(matches!(
    trans.send_retrieve_and_decode_one(&req, rb.tp_mut()).await.unwrap(),
    HistoryRes::One(_)
  ));
});

_create_http_test!(api(), vaccines, |rb, trans| async {
  let req = rb.vaccines(None, None, None).unwrap();
  assert!(matches!(
    trans.send_retrieve_and_decode_one(&req, rb.tp_mut()).await.unwrap(),
    VaccineRes::Many(_)
  ));
  let req = rb.vaccines(Some("pt"), None, None).unwrap();
  assert!(matches!(
    trans.send_retrieve_and_decode_one(&req, rb.tp_mut()).await.unwrap(),
    VaccineRes::One(_)
  ));
});

fn api() -> Covid19 {
  Covid19::from_origin("https://covid-api.mmediagroup.fr").unwrap()
}
