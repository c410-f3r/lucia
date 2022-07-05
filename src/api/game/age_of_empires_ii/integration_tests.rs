use crate::{
  api::{game::age_of_empires_ii::AgeOfEmpiresII, Api},
  network::Transport,
};

_create_http_test!(http(), civilization, |rb, trans| async {
  let req = rb.civilization(4).unwrap();
  let _ = trans.send_retrieve_and_decode_one(&req, rb.tp_mut()).await.unwrap();
});

_create_http_test!(http(), civilizations, |rb, trans| async {
  let req = rb.civilizations();
  let _ = trans.send_retrieve_and_decode_one(&req, rb.tp_mut()).await.unwrap();
});

_create_http_test!(http(), unit, |rb, trans| async {
  let req = rb.unit(4).unwrap();
  let _ = trans.send_retrieve_and_decode_one(&req, rb.tp_mut()).await.unwrap();
});

_create_http_test!(http(), units, |rb, trans| async {
  let req = rb.units();
  let _ = trans.send_retrieve_and_decode_one(&req, rb.tp_mut()).await.unwrap();
});

fn http() -> AgeOfEmpiresII {
  AgeOfEmpiresII::new("https://age-of-empires-2-api.herokuapp.com", ()).unwrap()
}
