use crate::gaming::age_of_empires_ii::AgeOfEmpiresII;
use lucia::{
  dnsn::SerdeJson,
  network::{transport::Transport, HttpParams},
};

_create_http_test!(AgeOfEmpiresII, http(), civilization, |pkgs_aux, trans| async {
  let pkg = &mut pkgs_aux.civilization().params(4).build();
  let _ = trans.send_retrieve_and_decode_contained(pkg, pkgs_aux).await.unwrap();
});

_create_http_test!(AgeOfEmpiresII, http(), civilizations, |pkgs_aux, trans| async {
  let pkg = &mut pkgs_aux.civilizations().build();
  let _ = trans.send_retrieve_and_decode_contained(pkg, pkgs_aux).await.unwrap();
});

_create_http_test!(AgeOfEmpiresII, http(), structure, |pkgs_aux, trans| async {
  let pkg = &mut pkgs_aux.structure().params(4).build();
  let _ = trans.send_retrieve_and_decode_contained(pkg, pkgs_aux).await.unwrap();
});

_create_http_test!(AgeOfEmpiresII, http(), structures, |pkgs_aux, trans| async {
  let pkg = &mut pkgs_aux.structures().build();
  let _ = trans.send_retrieve_and_decode_contained(pkg, pkgs_aux).await.unwrap();
});

_create_http_test!(AgeOfEmpiresII, http(), technology, |pkgs_aux, trans| async {
  let pkg = &mut pkgs_aux.technology().params(4).build();
  let _ = trans.send_retrieve_and_decode_contained(pkg, pkgs_aux).await.unwrap();
});

_create_http_test!(AgeOfEmpiresII, http(), technologies, |pkgs_aux, trans| async {
  let pkg = &mut pkgs_aux.technologies().build();
  let _ = trans.send_retrieve_and_decode_contained(pkg, pkgs_aux).await.unwrap();
});

_create_http_test!(AgeOfEmpiresII, http(), unit, |pkgs_aux, trans| async {
  let pkg = &mut pkgs_aux.unit().params(4).build();
  let _ = trans.send_retrieve_and_decode_contained(pkg, pkgs_aux).await.unwrap();
});

_create_http_test!(AgeOfEmpiresII, http(), units, |pkgs_aux, trans| async {
  let pkg = &mut pkgs_aux.units().build();
  let _ = trans.send_retrieve_and_decode_contained(pkg, pkgs_aux).await.unwrap();
});

fn http() -> (SerdeJson, HttpParams) {
  (SerdeJson, HttpParams::from_url("https://age-of-empires-2-api.herokuapp.com").unwrap())
}
