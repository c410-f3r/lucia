use crate::{api::calendar::nager_date::NagerDate, network::HttpMethod};
use alloc::vec::Vec;
use arrayvec::ArrayString;

type Res = Vec<V3LongWeekendRes>;

_create_json_endpoint! {
  NagerDate;

  V3LongWeekendReq<;;>

  |raw: Res| -> Res { Ok(raw) }

  V3LongWeekendParams(year: i16, country_code: &'rpd str) -> crate::Result<()> {
    |_hp| {
      _hp._method = HttpMethod::_Get;
      _hp._url_parts.set_path(format_args!("/api/v3/LongWeekend/{year}/{country_code}"))?;
    }
  }

  v3_long_weekend() {
    || {
      V3LongWeekendReq
    }
  }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V3LongWeekendRes {
  pub start_date: ArrayString<10>,
  pub end_date: ArrayString<10>,
  pub day_count: u8,
  pub need_bridge_day: bool,
}
