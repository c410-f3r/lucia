#![cfg(all(test, feature = "_integration-tests"))]

use crate::{
  api::calendar::nager_date::{
    V3AvailableCountriesParams, V3CountryInfoParams, V3LongWeekendParams,
    V3NextPublicHolidaysParams, V3NextPublicHolidaysWorldwideParams, V3PublicHolidaysParams,
  },
  network::{HttpParams, Transport},
};

_create_http_test!(http(), v3_available_countries, |rm, trans| async {
  let req = rm.v3_available_countries();
  let _ =
    trans.send_retrieve_and_decode_one(rm, &req, V3AvailableCountriesParams::new()).await.unwrap();
});

_create_http_test!(http(), v3_country_info, |rm, trans| async {
  let req = rm.v3_country_info();
  let _ =
    trans.send_retrieve_and_decode_one(rm, &req, V3CountryInfoParams::new("es")).await.unwrap();
});

_create_http_test!(http(), v3_long_weekend, |rm, trans| async {
  let req = rm.v3_long_weekend();
  let _ = trans
    .send_retrieve_and_decode_one(rm, &req, V3LongWeekendParams::new(2020, "es"))
    .await
    .unwrap();
});

_create_http_test!(http(), v3_next_public_holidays_worldwide, |rm, trans| async {
  let req = rm.v3_next_public_holidays_worldwide();
  let _ = trans
    .send_retrieve_and_decode_one(rm, &req, V3NextPublicHolidaysWorldwideParams::new())
    .await
    .unwrap();
});

_create_http_test!(http(), v3_next_public_holidays, |rm, trans| async {
  let req = rm.v3_next_public_holidays();
  let _ = trans
    .send_retrieve_and_decode_one(rm, &req, V3NextPublicHolidaysParams::new("es"))
    .await
    .unwrap();
});

_create_http_test!(http(), v3_public_holidays, |rm, trans| async {
  let req = rm.v3_public_holidays();
  let _ = trans
    .send_retrieve_and_decode_one(rm, &req, V3PublicHolidaysParams::new(2000, "es"))
    .await
    .unwrap();
});

fn http() -> HttpParams {
  HttpParams::from_origin("https://date.nager.at").unwrap()
}
