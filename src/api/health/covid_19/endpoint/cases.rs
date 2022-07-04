use crate::{
  api::health::covid_19::{endpoint::CountryInfo, Covid19},
  network::HttpMethod,
  utils::QueryParamWriter,
};
use alloc::{boxed::Box, collections::BTreeMap};
use arrayvec::ArrayString;

_create_json_endpoint! {
  Covid19;

  CasesReq<;;>

  |raw: CasesRes| -> CasesRes { Ok(raw) }

  cases(ab: Option<&str>, continent: Option<&str>, country: Option<&str>) -> crate::Result<:> {
    |api, tp| {
      tp.http_params._set(HttpMethod::Get, &api.origin);
      tp.http_params.url.try_push_str("/v1/cases")?;
      let _ = QueryParamWriter::new(&mut tp.http_params.url)
        .write_opt("ab", ab)?
        .write_opt("continent", continent)?
        .write_opt("country", country)?;
      CasesReq
    }
  }

  Ok
}

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum CasesRes {
  One(Box<OneCasesRes>),
  Many(BTreeMap<ArrayString<44>, OneCasesRes>),
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OneCasesRes {
  pub all: OneCaseAllRes,
  #[serde(flatten)]
  pub associations: BTreeMap<ArrayString<44>, OneCaseAssociationRes>,
}

#[derive(Debug, serde::Deserialize)]
pub struct OneCaseAllRes {
  pub confirmed: u32,
  pub recovered: u32,
  pub deaths: u32,
  #[serde(flatten)]
  pub country_info: CountryInfo,
}

#[derive(Debug, serde::Deserialize)]
pub struct OneCaseAssociationRes {
  pub lat: ArrayString<12>,
  pub long: ArrayString<12>,
  pub confirmed: u32,
  pub recovered: u32,
  pub deaths: u32,
  pub updated: ArrayString<20>,
}
