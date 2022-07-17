use crate::{
  api::calendar::nager_date::{NagerDate, V3PublicHoliday},
  network::HttpMethod,
};
use alloc::vec::Vec;

type Res = Vec<V3PublicHoliday>;

_create_json_endpoint! {
  NagerDate;

  V3PublicHolidaysReq<;;>

  |raw: Res| -> Res { Ok(raw) }

  V3PublicHolidaysParams(year: i32, country_code: &'rpd str) -> crate::Result<()> {
    |_hp| {
      _hp._method = HttpMethod::_Get;
      _hp._url_parts.set_path(format_args!("/api/v3/PublicHolidays/{year}/{country_code}"))?;
    }
  }

  v3_public_holidays() {
    || {
      V3PublicHolidaysReq
    }
  }
}
