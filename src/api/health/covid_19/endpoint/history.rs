use crate::{
  api::health::covid_19::{CountryInfo, Covid19},
  network::HttpMethod,
};
use alloc::{boxed::Box, collections::BTreeMap};
use arrayvec::ArrayString;

_create_json_endpoint! {
  Covid19;

  HistoryReq<;;>

  |raw: HistoryRes| -> HistoryRes { Ok(raw) }

  HistoryParams(
    hs: HistoryStatus, ab: Option<&'rpd str>, continent: Option<&'rpd str>, country: Option<&'rpd str>
  ) -> crate::Result<()> {
    |hp| {
      hp._method = HttpMethod::_Get;
      hp._url_parts.set_path(format_args!("/v1/history"))?;
      let _ = hp._url_parts.query_writer()
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
