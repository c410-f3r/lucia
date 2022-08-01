use crate::{
  api::calendar::nager_date::NagerDate,
  data_format::{JsonRequest, JsonResponse},
  network::http::Method,
};
use alloc::{boxed::Box, vec::Vec};
use arrayvec::ArrayString;

type Res = Box<V3CountryInfoRes>;

_create_endpoint! {
  NagerDate => JsonResponse|JsonRequest|_json_request;

  V3CountryInfoReq<;;>

  |raw: Res, _resp| -> Res { Ok(raw) }

  V3CountryInfoParams(country: &'reqp str) -> crate::Result<()> {
    |_hp| {
      _hp.tp._method = Method::Get;
      _hp.tp._url_parts.set_path(format_args!("/api/v3/CountryInfo/{}", country))?;
    }
  }

  v3_country_info() {
    || {
      V3CountryInfoReq
    }
  }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct V3CountryInfoRes {
  pub common_name: ArrayString<12>,
  pub official_name: ArrayString<26>,
  pub country_code: ArrayString<12>,
  pub region: ArrayString<6>,
  pub borders: Option<Vec<V3CountryInfoRes>>,
}
