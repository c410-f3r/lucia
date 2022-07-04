use crate::{
  api::health::covid_19::{endpoint::CountryInfo, Covid19},
  network::HttpMethod,
  utils::QueryParamWriter,
};
use alloc::{boxed::Box, collections::BTreeMap};
use arrayvec::ArrayString;

_create_json_endpoint! {
  Covid19;

  HistoryReq<;;>

  |raw: HistoryRes| -> HistoryRes { Ok(raw) }

  history(
    hs: HistoryStatus, ab: Option<&str>, continent: Option<&str>, country: Option<&str>
  ) -> crate::Result<:> {
    |api, tp| {
      tp.http_params._set(HttpMethod::Get, &api.origin);
      tp.http_params.url.try_push_str("/v1/history")?;
      let _ = QueryParamWriter::new(&mut tp.http_params.url)
        .write("status", match hs {
          HistoryStatus::Confirmed => "Confirmed",
          HistoryStatus::Deaths => "Deaths",
        })?
        .write_opt("ab", ab)?
        .write_opt("continent", continent)?
        .write_opt("country", country)?;
      HistoryReq
    }
  }

  Ok
}

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum HistoryRes {
  One(Box<OneHistoryRes>),
  Many(BTreeMap<ArrayString<44>, OneHistoryRes>),
}

#[derive(Debug)]
pub enum HistoryStatus {
  Confirmed,
  Deaths,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OneHistoryRes {
  pub all: OneHistoryAllRes,
}

#[derive(Debug, serde::Deserialize)]
pub struct OneHistoryAllRes {
  #[serde(flatten)]
  pub country_info: CountryInfo,
  pub dates: BTreeMap<ArrayString<15>, u32>,
}
