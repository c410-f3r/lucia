use crate::calendar::nager_date::NagerDate;
use alloc::vec::Vec;
use arrayvec::ArrayString;
use lucia::{
  data_formats::{JsonRequest, JsonResponse},
  network::http::Method,
};

type Res = Vec<V3AvailableCountriesRes>;

_create_endpoint! {
  NagerDate => JsonResponse|JsonRequest|json_request;

  V3AvailableCountriesReq<;;>

  |raw: Res, _resp| -> Res { Ok(raw) }

  V3AvailableCountriesParams() -> crate::Result<()> {
    |_hp| {
      _hp.tp.method = Method::Get;
      _hp.tp.url_parts.set_path(format_args!("/api/v3/AvailableCountries"))?;
    }
  }

  v3_available_countries() {
    || {
      V3AvailableCountriesReq
    }
  }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct V3AvailableCountriesRes {
  pub country_code: ArrayString<2>,
  pub name: ArrayString<22>,
}
