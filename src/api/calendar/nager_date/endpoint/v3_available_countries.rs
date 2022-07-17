use crate::{api::calendar::nager_date::NagerDate, network::HttpMethod};
use alloc::vec::Vec;
use arrayvec::ArrayString;

type Res = Vec<V3AvailableCountriesRes>;

_create_json_endpoint! {
  NagerDate;

  V3AvailableCountriesReq<;;>

  |raw: Res| -> Res { Ok(raw) }

  V3AvailableCountriesParams() -> crate::Result<()> {
    |_hp| {
      _hp._method = HttpMethod::_Get;
      _hp._url_parts.set_path(format_args!("/api/v3/AvailableCountries"))?;
    }
  }

  v3_available_countries() {
    || {
      V3AvailableCountriesReq
    }
  }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct V3AvailableCountriesRes {
  pub country_code: ArrayString<2>,
  pub name: ArrayString<22>,
}
