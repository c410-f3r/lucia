use crate::gaming::age_of_empires_ii::{
  CivilizationParams, CivilizationsParams, StructureParams, StructuresParams, TechnologiesParams,
  TechnologyParams, UnitParams, UnitsParams,
};
use lucia::{
  dnsn::SerdeJson,
  misc::CommonParams,
  network::{http::ReqParams, Transport},
};

_create_http_test!(http(), civilization, |rmw, trans| async {
  let req = rmw.civilization();
  let _ = trans.send_retrieve_and_decode_one(rmw, &req, CivilizationParams::new(4)).await.unwrap();
});

_create_http_test!(http(), civilizations, |rmw, trans| async {
  let req = rmw.civilizations();
  let _ = trans.send_retrieve_and_decode_one(rmw, &req, CivilizationsParams::new()).await.unwrap();
});

_create_http_test!(http(), structure, |rmw, trans| async {
  let req = rmw.structure();
  let _ = trans.send_retrieve_and_decode_one(rmw, &req, StructureParams::new(4)).await.unwrap();
});

_create_http_test!(http(), structures, |rmw, trans| async {
  let req = rmw.structures();
  let _ = trans.send_retrieve_and_decode_one(rmw, &req, StructuresParams::new()).await.unwrap();
});

_create_http_test!(http(), technology, |rmw, trans| async {
  let req = rmw.technology();
  let _ = trans.send_retrieve_and_decode_one(rmw, &req, TechnologyParams::new(4)).await.unwrap();
});

_create_http_test!(http(), technologies, |rmw, trans| async {
  let req = rmw.technologies();
  let _ = trans.send_retrieve_and_decode_one(rmw, &req, TechnologiesParams::new()).await.unwrap();
});

_create_http_test!(http(), unit, |rmw, trans| async {
  let req = rmw.unit();
  let _ = trans.send_retrieve_and_decode_one(rmw, &req, UnitParams::new(4)).await.unwrap();
});

_create_http_test!(http(), units, |rmw, trans| async {
  let req = rmw.units();
  let _ = trans.send_retrieve_and_decode_one(rmw, &req, UnitsParams::new()).await.unwrap();
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
