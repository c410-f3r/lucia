use crate::{
  api::health::m_media_covid_19::{CountryInfo, MMediaCovid19},
  data_format::{JsonRequest, JsonResponse},
  network::http::Method,
};
use alloc::{boxed::Box, collections::BTreeMap};
use arrayvec::ArrayString;

_create_endpoint! {
  MMediaCovid19 => JsonResponse|JsonRequest|_json_request;

  HistoryReq<;;>

  |raw: HistoryRes, _resp| -> HistoryRes { Ok(raw) }

  HistoryParams(
    hs: HistoryStatus, ab: Option<&'reqp str>, continent: Option<&'reqp str>, country: Option<&'reqp str>
  ) -> crate::Result<()> {
    |hp| {
      hp.tp._method = Method::Get;
      hp.tp._url_parts.set_path(format_args!("/v1/history"))?;
      let _ = hp.tp._url_parts.query_writer()
        .write("status", match hs {
          HistoryStatus::Confirmed => "Confirmed",
          HistoryStatus::Deaths => "Deaths",
        })?
        .write_opt("ab", ab)?
        .write_opt("continent", continent)?
        .write_opt("country", country)?;
    }
  }

  history() {
    || {
      HistoryReq
    }
  }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[derive(Debug)]
pub enum HistoryRes {
  One(Box<OneHistoryRes>),
  Many(BTreeMap<ArrayString<44>, OneHistoryRes>),
}

#[derive(Debug)]
pub enum HistoryStatus {
  Confirmed,
  Deaths,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "PascalCase"))]
#[derive(Debug)]
pub struct OneHistoryRes {
  pub all: OneHistoryAllRes,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Debug)]
pub struct OneHistoryAllRes {
  #[cfg_attr(feature = "serde", serde(flatten))]
  pub country_info: CountryInfo,
  pub dates: BTreeMap<ArrayString<15>, u32>,
}
