use crate::{
  api::health::covid_19::{CountryInfo, Covid19},
  network::HttpMethod,
};
use alloc::{boxed::Box, collections::BTreeMap};
use arrayvec::ArrayString;

_create_json_endpoint! {
  Covid19;

  VaccinesReq<;;>

  |raw: VaccineRes| -> VaccineRes { Ok(raw) }

  VaccinesParams(
    ab: Option<&'rpd str>, continent: Option<&'rpd str>, country: Option<&'rpd str>
  ) -> crate::Result<()> {
    |hp| {
      hp._method = HttpMethod::_Get;
      hp._url_parts.set_path(format_args!("/v1/vaccines"))?;
      let _ = hp._url_parts.query_writer()
        .write_opt("ab", ab)?
        .write_opt("continent", continent)?
        .write_opt("country", country)?;
    }
  }

  vaccines() {
    || {
      VaccinesReq
    }
  }
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
