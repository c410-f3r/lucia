use crate::{
  api::calendar::nager_date::NagerDate,
  data_format::{JsonRequest, JsonResponse},
  network::http::Method,
};
use alloc::vec::Vec;
use arrayvec::ArrayString;

type Res = Vec<V3LongWeekendRes>;

_create_endpoint! {
  NagerDate => JsonResponse|JsonRequest|_json_request;

  V3LongWeekendReq<;;>

  |raw: Res, _resp| -> Res { Ok(raw) }

  V3LongWeekendParams(year: i16, country_code: &'reqp str) -> crate::Result<()> {
    |_hp| {
      _hp.tp._method = Method::Get;
      _hp.tp._url_parts.set_path(format_args!("/api/v3/LongWeekend/{year}/{country_code}"))?;
    }
  }

  v3_long_weekend() {
    || {
      V3LongWeekendReq
    }
  }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct V3LongWeekendRes {
  pub start_date: ArrayString<10>,
  pub end_date: ArrayString<10>,
  pub day_count: u8,
  pub need_bridge_day: bool,
}
