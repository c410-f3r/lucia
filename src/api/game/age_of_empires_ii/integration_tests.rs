#![cfg(all(test, feature = "_integration-tests"))]

use crate::{
  api::game::age_of_empires_ii::{
    CivilizationParams, CivilizationsParams, UnitParams, UnitsParams,
  },
  network::{HttpParams, Transport},
};

_create_http_test!(http(), civilization, |rm, trans| async {
  let req = rm.civilization();
  let _ = trans.send_retrieve_and_decode_one(rm, &req, CivilizationParams::new(4)).await.unwrap();
});

_create_http_test!(http(), civilizations, |rm, trans| async {
  let req = rm.civilizations();
  let _ = trans.send_retrieve_and_decode_one(rm, &req, CivilizationsParams::new()).await.unwrap();
});

_create_http_test!(http(), unit, |rm, trans| async {
  let req = rm.unit();
  let _ = trans.send_retrieve_and_decode_one(rm, &req, UnitParams::new(4)).await.unwrap();
});

_create_http_test!(http(), units, |rm, trans| async {
  let req = rm.units();
  let _ = trans.send_retrieve_and_decode_one(rm, &req, UnitsParams::new()).await.unwrap();
});

fn http() -> HttpParams {
  HttpParams::from_origin("https://age-of-empires-2-api.herokuapp.com").unwrap()
}
