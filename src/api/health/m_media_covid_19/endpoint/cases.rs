use crate::{
  api::health::m_media_covid_19::{CountryInfo, MMediaCovid19},
  data_format::{JsonRequest, JsonResponse},
  network::http::Method,
};
use alloc::{boxed::Box, collections::BTreeMap};
use arrayvec::ArrayString;

_create_endpoint! {
  MMediaCovid19 => JsonResponse|JsonRequest|_json_request;

  CasesReq<;;>

  |raw: CasesRes, _resp| -> CasesRes { Ok(raw) }

  CasesParams(ab: Option<&'reqp str>, continent: Option<&'reqp str>, country: Option<&'reqp str>) -> crate::Result<()> {
    |hp| {
      hp.tp._method = Method::Get;
      hp.tp._url_parts.set_path(format_args!("/v1/cases"))?;
      let _ = hp.tp._url_parts.query_writer()
        .write_opt("ab", ab)?
        .write_opt("continent", continent)?
        .write_opt("country", country)?;
    }
  }

  cases() {
    || {
      CasesReq
    }
  }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[derive(Debug)]
pub enum CasesRes {
  One(Box<OneCasesRes>),
  Many(BTreeMap<ArrayString<44>, OneCasesRes>),
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
#[derive(Debug)]
pub struct OneCasesRes {
  pub all: OneCaseAllRes,
  #[cfg_attr(feature = "serde", serde(flatten))]
  pub associations: BTreeMap<ArrayString<44>, OneCaseAssociationRes>,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Debug)]
pub struct OneCaseAllRes {
  pub confirmed: u32,
  pub recovered: u32,
  pub deaths: u32,
  #[cfg_attr(feature = "serde", serde(flatten))]
  pub country_info: CountryInfo,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Debug)]
pub struct OneCaseAssociationRes {
  pub lat: ArrayString<12>,
  pub long: ArrayString<12>,
  pub confirmed: u32,
  pub recovered: u32,
  pub deaths: u32,
  pub updated: ArrayString<20>,
}
