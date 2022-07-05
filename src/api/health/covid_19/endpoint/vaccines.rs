use crate::{
  api::health::covid_19::{endpoint::CountryInfo, Covid19},
  network::HttpMethod,
  utils::_QueryParamWriter,
};
use alloc::{boxed::Box, collections::BTreeMap};
use arrayvec::ArrayString;

_create_json_endpoint! {
  Covid19;

  VaccinesReq<;;>

  |raw: VaccineRes| -> VaccineRes { Ok(raw) }

  vaccines(
    ab: Option<&str>, continent: Option<&str>, country: Option<&str>
  ) -> crate::Result<:> {
    |api, tp| {
      tp._http_params._set(HttpMethod::Get, None, &api.origin);
      tp._http_params._url.try_push_str("/v1/vaccines")?;
      let _ = _QueryParamWriter::_new(&mut tp._http_params._url)
        ._write_opt("ab", ab)?
        ._write_opt("continent", continent)?
        ._write_opt("country", country)?;
      VaccinesReq
    }
  }

  Ok
}

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum VaccineRes {
  One(Box<OneVaccineRes>),
  Many(BTreeMap<ArrayString<44>, OneVaccineRes>),
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OneVaccineRes {
  pub all: OneVaccineAllRes,
}

#[derive(Debug, serde::Deserialize)]
pub struct OneVaccineAllRes {
  pub administered: u64,
  pub people_vaccinated: u64,
  pub people_partially_vaccinated: u64,
  #[serde(flatten)]
  pub country_info: CountryInfo,
  pub updated: Option<ArrayString<35>>,
}
