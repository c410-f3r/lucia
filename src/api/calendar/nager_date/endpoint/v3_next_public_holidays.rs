use crate::{
  api::calendar::nager_date::{NagerDate, V3PublicHoliday},
  data_format::{JsonRequest, JsonResponse},
  network::http::Method,
};
use alloc::vec::Vec;

type Res = Vec<V3NextPublicHoliday>;

_create_endpoint! {
  NagerDate => JsonResponse|JsonRequest|_json_request;

  V3NextPublicHolidaysReq<;;>

  |raw: Res, _resp| -> Res { Ok(raw) }

  V3NextPublicHolidaysParams(country_code: &'reqp str) -> crate::Result<()> {
    |_hp| {
      _hp.tp._method = Method::Get;
      _hp.tp._url_parts.set_path(format_args!("/api/v3/NextPublicHolidays/{country_code}"))?;
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
