use crate::calendar::nager_date::{NagerDate, V3PublicHoliday};
use alloc::vec::Vec;
use lucia::{
  data_formats::{JsonRequest, JsonResponse},
  network::http::Method,
};

type Res = Vec<V3NextPublicHoliday>;

_create_endpoint! {
  NagerDate => JsonResponse|JsonRequest|json_request;

  V3NextPublicHolidaysReq<;;>

  |raw: Res, _resp| -> Res { Ok(raw) }

  V3NextPublicHolidaysParams(country_code: &'reqp str) -> crate::Result<()> {
    |_hp| {
      _hp.tp.method = Method::Get;
      _hp.tp.url_parts.set_path(format_args!("/api/v3/NextPublicHolidays/{country_code}"))?;
    }
  }

  v3_next_public_holidays() {
    || {
      V3NextPublicHolidaysReq
    }
  }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Debug)]
pub struct V3NextPublicHoliday(V3PublicHoliday);
