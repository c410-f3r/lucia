use crate::calendar::nager_date::{NagerDate, V3PublicHoliday};
use alloc::vec::Vec;
use lucia::{
  data_formats::{JsonRequest, JsonResponse},
  network::http::Method,
};

type Res = Vec<V3PublicHolidayWorldwide>;

_create_endpoint! {
  NagerDate => JsonResponse|JsonRequest|json_request;

  V3NextPublicHolidaysWorldwideReq<;;>

  |raw: Res, _resp| -> Res { Ok(raw) }

  V3NextPublicHolidaysWorldwideParams() -> crate::Result<()> {
    |_hp| {
      _hp.tp.method = Method::Get;
      _hp.tp.url_parts.set_path(format_args!("/api/v3/NextPublicHolidaysWorldwide"))?;
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
