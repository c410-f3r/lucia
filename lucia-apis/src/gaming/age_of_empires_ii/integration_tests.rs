use crate::gaming::age_of_empires_ii::{
  CivilizationParams, CivilizationsParams, StructureParams, StructuresParams, TechnologiesParams,
  TechnologyParams, UnitParams, UnitsParams,
};
use lucia::{
  dnsn::SerdeJson,
  misc::CommonParams,
  network::{http::ReqParams, Transport},
};

_create_http_test!(http(), civilization, |rm, trans| async {
  let req = rm.civilization();
  let _ = trans.send_retrieve_and_decode_one(rm, &req, CivilizationParams::new(4)).await.unwrap();
});

_create_http_test!(http(), civilizations, |rm, trans| async {
  let req = rm.civilizations();
  let _ = trans.send_retrieve_and_decode_one(rm, &req, CivilizationsParams::new()).await.unwrap();
});

_create_http_test!(http(), structure, |rm, trans| async {
  let req = rm.structure();
  let _ = trans.send_retrieve_and_decode_one(rm, &req, StructureParams::new(4)).await.unwrap();
});

_create_http_test!(http(), structures, |rm, trans| async {
  let req = rm.structures();
  let _ = trans.send_retrieve_and_decode_one(rm, &req, StructuresParams::new()).await.unwrap();
});

_create_http_test!(http(), technology, |rm, trans| async {
  let req = rm.technology();
  let _ = trans.send_retrieve_and_decode_one(rm, &req, TechnologyParams::new(4)).await.unwrap();
});

_create_http_test!(http(), technologies, |rm, trans| async {
  let req = rm.technologies();
  let _ = trans.send_retrieve_and_decode_one(rm, &req, TechnologiesParams::new()).await.unwrap();
});

_create_http_test!(http(), unit, |rm, trans| async {
  let req = rm.unit();
  let _ = trans.send_retrieve_and_decode_one(rm, &req, UnitParams::new(4)).await.unwrap();
});

_create_http_test!(http(), units, |rm, trans| async {
  let req = rm.units();
  let _ = trans.send_retrieve_and_decode_one(rm, &req, UnitsParams::new()).await.unwrap();
});

fn http() -> (CommonParams<ReqParams, ()>, SerdeJson) {
  (
    CommonParams::new(
      ReqParams::from_origin("https://age-of-empires-2-api.herokuapp.com").unwrap(),
      (),
    ),
    SerdeJson::default(),
  )
}
