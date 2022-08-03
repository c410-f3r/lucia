use crate::calendar::nager_date::{NagerDate, V3PublicHoliday};
use alloc::vec::Vec;
use lucia::{
  data_formats::{JsonRequest, JsonResponse},
  network::http::Method,
};

type Res = Vec<V3PublicHoliday>;

_create_endpoint! {
  NagerDate => JsonResponse|JsonRequest|json_request;

  V3PublicHolidaysReq<;;>

  |raw: Res, _resp| -> Res { Ok(raw) }

  V3PublicHolidaysParams(year: i32, country_code: &'reqp str) -> crate::Result<()> {
    |_hp| {
      _hp.tp.method = Method::Get;
      _hp.tp.url_parts.set_path(format_args!("/api/v3/PublicHolidays/{year}/{country_code}"))?;
    }
  }

  v3_public_holidays() {
    || {
      V3PublicHolidaysReq
    }
  }
}
