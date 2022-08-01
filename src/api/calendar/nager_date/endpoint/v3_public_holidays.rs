use crate::{
  api::calendar::nager_date::{NagerDate, V3PublicHoliday},
  data_format::{JsonRequest, JsonResponse},
  network::http::Method,
};
use alloc::vec::Vec;

type Res = Vec<V3PublicHoliday>;

_create_endpoint! {
  NagerDate => JsonResponse|JsonRequest|_json_request;

  V3PublicHolidaysReq<;;>

  |raw: Res, _resp| -> Res { Ok(raw) }

  V3PublicHolidaysParams(year: i32, country_code: &'reqp str) -> crate::Result<()> {
    |_hp| {
      _hp.tp._method = Method::Get;
      _hp.tp._url_parts.set_path(format_args!("/api/v3/PublicHolidays/{year}/{country_code}"))?;
    }
  }

  v3_public_holidays() {
    || {
      V3PublicHolidaysReq
    }
  }
}
