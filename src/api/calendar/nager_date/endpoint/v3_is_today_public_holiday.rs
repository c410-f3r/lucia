use crate::{
  api::calendar::nager_date::NagerDate,
  data_format::{JsonRequest, JsonResponse},
  network::http::{Method, StatusCode},
};

_create_endpoint! {
  NagerDate => JsonResponse|JsonRequest|_json_request;

  V3IsTodayPublicHolidayReq<;;>

  |raw: (), resp| -> () {
    if resp.into().status_code == StatusCode::Ok {
      Ok(raw)
    }
    else {
      Err(crate::Error::IncompatibleStatusCode(StatusCode::Ok, resp.into().status_code))
    }
  }

  V3IsTodayPublicHolidayParams(country_code: &'reqp str, county_code: Option<&'reqp str>, offset: Option<i8>) -> crate::Result<()> {
    |_hp| {
      _hp.tp._method = Method::Get;
      _hp.tp._url_parts.set_path(format_args!("/api/v3/IsTodayPublicHoliday/{country_code}"))?;
      let _ = _hp.tp._url_parts.query_writer()
        .write_opt("countyCode", county_code)?
        .write_opt("offset", offset)?;
    }
  }

  v3_is_today_public_holiday() {
    || {
      V3IsTodayPublicHolidayReq
    }
  }
}
