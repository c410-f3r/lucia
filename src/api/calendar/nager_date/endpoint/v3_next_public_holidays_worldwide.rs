use crate::{
  api::calendar::nager_date::{NagerDate, V3PublicHoliday},
  data_format::{JsonRequest, JsonResponse},
  network::http::Method,
};
use alloc::vec::Vec;

type Res = Vec<V3PublicHolidayWorldwide>;

_create_endpoint! {
  NagerDate => JsonResponse|JsonRequest|_json_request;

  V3NextPublicHolidaysWorldwideReq<;;>

  |raw: Res, _resp| -> Res { Ok(raw) }

  V3NextPublicHolidaysWorldwideParams() -> crate::Result<()> {
    |_hp| {
      _hp.tp._method = Method::Get;
      _hp.tp._url_parts.set_path(format_args!("/api/v3/NextPublicHolidaysWorldwide"))?;
    }
  }

  v3_next_public_holidays_worldwide() {
    || {
      V3NextPublicHolidaysWorldwideReq
    }
  }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Debug)]
pub struct V3PublicHolidayWorldwide(V3PublicHoliday);
