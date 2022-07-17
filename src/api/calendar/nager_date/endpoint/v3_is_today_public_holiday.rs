use crate::{api::calendar::nager_date::NagerDate, network::HttpMethod};

_create_json_endpoint! {
  NagerDate;

  V3IsTodayPublicHolidayReq<;;>

  |raw: ()| -> () { Ok(raw) }

  V3IsTodayPublicHolidayParams(country_code: &'rpd str, county_code: Option<&'rpd str>, offset: Option<i8>) -> crate::Result<()> {
    |_hp| {
      _hp._method = HttpMethod::_Get;
      _hp._url_parts.set_path(format_args!("/api/v3/IsTodayPublicHoliday/{country_code}"))?;
      let _ = _hp._url_parts.query_writer()
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
