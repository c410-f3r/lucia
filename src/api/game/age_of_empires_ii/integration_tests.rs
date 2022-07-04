use crate::{api::game::age_of_empires_ii::AgeOfEmpiresII, network::Transport, Api};

_create_http_test!(api(), civilization, |rb, trans| async {
  let req = rb.civilization(4).unwrap();
  let _ = trans.send_retrieve_and_decode_one(&req, rb.tp_mut()).await.unwrap();
});

_create_http_test!(api(), civilizations, |rb, trans| async {
  let req = rb.civilizations();
  let _ = trans.send_retrieve_and_decode_one(&req, rb.tp_mut()).await.unwrap();
});

_create_http_test!(api(), unit, |rb, trans| async {
  let req = rb.unit(4).unwrap();
  let _ = trans.send_retrieve_and_decode_one(&req, rb.tp_mut()).await.unwrap();
});

_create_http_test!(api(), units, |rb, trans| async {
  let req = rb.units();
  let _ = trans.send_retrieve_and_decode_one(&req, rb.tp_mut()).await.unwrap();
});

fn api() -> AgeOfEmpiresII {
  AgeOfEmpiresII::from_origin("https://age-of-empires-2-api.herokuapp.com").unwrap()
}
