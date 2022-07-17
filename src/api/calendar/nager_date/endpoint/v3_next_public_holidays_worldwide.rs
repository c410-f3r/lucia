use crate::{
  api::calendar::nager_date::{NagerDate, V3PublicHoliday},
  network::HttpMethod,
};
use alloc::vec::Vec;

type Res = Vec<V3PublicHoliday>;

_create_json_endpoint! {
  NagerDate;

  V3NextPublicHolidaysWorldwideReq<;;>

  |raw: Res| -> Res { Ok(raw) }

  V3NextPublicHolidaysWorldwideParams() -> crate::Result<()> {
    |_hp| {
      _hp._method = HttpMethod::_Get;
      _hp._url_parts.set_path(format_args!("/api/v3/NextPublicHolidaysWorldwide"))?;
    }
  }

  v3_next_public_holidays_worldwide() {
    || {
      V3NextPublicHolidaysWorldwideReq
    }
  }
}
