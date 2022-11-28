use crate::calendar::nager_date::NagerDate;
use lucia::{
  dnsn::SerdeJson,
  network::{transport::Transport, HttpParams},
};

_create_http_test!(NagerDate, http(), v3_available_countries, |pkgs_aux, trans| async {
  let pkg = &mut pkgs_aux.v3_available_countries().build();
  let _ = trans.send_retrieve_and_decode_contained(pkg, pkgs_aux).await.unwrap();
});

_create_http_test!(NagerDate, http(), v3_country_info, |pkgs_aux, trans| async {
  let pkg = &mut pkgs_aux.v3_country_info().params("es").build();
  let _ = trans.send_retrieve_and_decode_contained(pkg, pkgs_aux).await.unwrap();
});

_create_http_test!(NagerDate, http(), v3_long_weekend, |pkgs_aux, trans| async {
  let pkg = &mut pkgs_aux.v3_long_weekend().params(2020, "es").build();
  let _ = trans.send_retrieve_and_decode_contained(pkg, pkgs_aux).await.unwrap();
});

_create_http_test!(NagerDate, http(), v3_next_public_holidays_worldwide, |pkgs_aux, trans| async {
  let pkg = &mut pkgs_aux.v3_next_public_holidays_worldwide().build();
  let _ = trans.send_retrieve_and_decode_contained(pkg, pkgs_aux).await.unwrap();
});

_create_http_test!(NagerDate, http(), v3_next_public_holidays, |pkgs_aux, trans| async {
  let pkg = &mut pkgs_aux.v3_next_public_holidays().params("es").build();
  let _ = trans.send_retrieve_and_decode_contained(pkg, pkgs_aux).await.unwrap();
});

_create_http_test!(NagerDate, http(), v3_public_holidays, |pkgs_aux, trans| async {
  let pkg = &mut pkgs_aux.v3_public_holidays().params(2000, "es").build();
  let _ = trans.send_retrieve_and_decode_contained(pkg, pkgs_aux).await.unwrap();
});

fn http() -> (SerdeJson, HttpParams) {
  (SerdeJson, HttpParams::from_url("https://date.nager.at").unwrap())
}
