use crate::health::m_media_covid_19::{CountryInfo, MMediaCovid19};
use alloc::{boxed::Box, collections::BTreeMap};
use arrayvec::ArrayString;
use lucia::{
  data_formats::{JsonRequest, JsonResponse},
  network::http::Method,
};

_create_endpoint! {
  MMediaCovid19 => JsonResponse|JsonRequest|json_request;

  VaccinesReq<;;>

  |raw: VaccineRes, _resp| -> VaccineRes { Ok(raw) }

  VaccinesParams(
    ab: Option<&'reqp str>, continent: Option<&'reqp str>, country: Option<&'reqp str>
  ) -> crate::Result<()> {
    |hp| {
      hp.tp.method = Method::Get;
      hp.tp.url_parts.set_path(format_args!("/v1/vaccines"))?;
      let _ = hp.tp.url_parts.query_writer()
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

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[derive(Debug)]
pub enum VaccineRes {
  One(Box<OneVaccineRes>),
  Many(BTreeMap<ArrayString<44>, OneVaccineRes>),
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
#[derive(Debug)]
pub struct OneVaccineRes {
  pub all: OneVaccineAllRes,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Debug)]
pub struct OneVaccineAllRes {
  pub administered: u64,
  pub people_vaccinated: u64,
  pub people_partially_vaccinated: u64,
  #[cfg_attr(feature = "serde", serde(flatten))]
  pub country_info: CountryInfo,
  pub updated: Option<ArrayString<35>>,
}
