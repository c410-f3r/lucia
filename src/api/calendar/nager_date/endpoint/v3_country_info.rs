use crate::{api::calendar::nager_date::NagerDate, network::HttpMethod};
use alloc::vec::Vec;
use arrayvec::ArrayString;

type Res = V3CountryInfoRes;

_create_json_endpoint! {
  NagerDate;

  V3CountryInfoReq<;;>

  |raw: Res| -> Res { Ok(raw) }

  V3CountryInfoParams(country: &'rpd str) -> crate::Result<()> {
    |_hp| {
      _hp._method = HttpMethod::_Get;
      _hp._url_parts.set_path(format_args!("/api/v3/CountryInfo/{}", country))?;
    }
  }

  v3_country_info() {
    || {
      V3CountryInfoReq
    }
  }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V3CountryInfoRes {
  pub common_name: ArrayString<12>,
  pub official_name: ArrayString<26>,
  pub country_code: ArrayString<12>,
  pub region: ArrayString<6>,
  pub borders: Option<Vec<V3CountryInfoRes>>,
}
