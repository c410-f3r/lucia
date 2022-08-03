use crate::calendar::nager_date::NagerDate;
use alloc::vec::Vec;
use arrayvec::ArrayString;
use lucia::{
  data_formats::{JsonRequest, JsonResponse},
  network::http::Method,
};

type Res = Vec<V3LongWeekendRes>;

_create_endpoint! {
  NagerDate => JsonResponse|JsonRequest|json_request;

  V3LongWeekendReq<;;>

  |raw: Res, _resp| -> Res { Ok(raw) }

  V3LongWeekendParams(year: i16, country_code: &'reqp str) -> crate::Result<()> {
    |_hp| {
      _hp.tp.method = Method::Get;
      _hp.tp.url_parts.set_path(format_args!("/api/v3/LongWeekend/{year}/{country_code}"))?;
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
