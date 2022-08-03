use crate::calendar::nager_date::{
  V3AvailableCountriesParams, V3CountryInfoParams, V3LongWeekendParams, V3NextPublicHolidaysParams,
  V3NextPublicHolidaysWorldwideParams, V3PublicHolidaysParams,
};
use lucia::{
  dnsn::SerdeJson,
  misc::CommonParams,
  network::{http::ReqParams, Transport},
};

_create_http_test!(http(), v3_available_countries, |rmw, trans| async {
  let req = rmw.v3_available_countries();
  let _ =
    trans.send_retrieve_and_decode_one(rmw, &req, V3AvailableCountriesParams::new()).await.unwrap();
});

_create_http_test!(http(), v3_country_info, |rmw, trans| async {
  let req = rmw.v3_country_info();
  let _ =
    trans.send_retrieve_and_decode_one(rmw, &req, V3CountryInfoParams::new("es")).await.unwrap();
});

_create_http_test!(http(), v3_long_weekend, |rmw, trans| async {
  let req = rmw.v3_long_weekend();
  let _ = trans
    .send_retrieve_and_decode_one(rmw, &req, V3LongWeekendParams::new(2020, "es"))
    .await
    .unwrap();
});

_create_http_test!(http(), v3_next_public_holidays_worldwide, |rmw, trans| async {
  let req = rmw.v3_next_public_holidays_worldwide();
  let _ = trans
    .send_retrieve_and_decode_one(rmw, &req, V3NextPublicHolidaysWorldwideParams::new())
    .await
    .unwrap();
});

_create_http_test!(http(), v3_next_public_holidays, |rmw, trans| async {
  let req = rmw.v3_next_public_holidays();
  let _ = trans
    .send_retrieve_and_decode_one(rmw, &req, V3NextPublicHolidaysParams::new("es"))
    .await
    .unwrap();
});

_create_http_test!(http(), v3_public_holidays, |rmw, trans| async {
  let req = rmw.v3_public_holidays();
  let _ = trans
    .send_retrieve_and_decode_one(rmw, &req, V3PublicHolidaysParams::new(2000, "es"))
    .await
    .unwrap();
});

fn http() -> (CommonParams<ReqParams, ()>, SerdeJson) {
  (
    CommonParams::new(ReqParams::from_origin("https://date.nager.at").unwrap(), ()),
    SerdeJson::default(),
  )
}
