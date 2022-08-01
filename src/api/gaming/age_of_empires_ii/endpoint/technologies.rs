use crate::{
  api::gaming::age_of_empires_ii::{AgeOfEmpiresII, TechnologyRes},
  data_format::{JsonRequest, JsonResponse},
  network::http::Method,
};
use alloc::vec::Vec;

_create_endpoint! {
  AgeOfEmpiresII => JsonResponse|JsonRequest|_json_request;

  TechnologiesReq<;;>

  |raw: TechnologiesRes, _resp| -> TechnologiesRes { Ok(raw) }

  TechnologiesParams() -> crate::Result<()> {
    |hp| {
      hp.tp._method = Method::Get;
      hp.tp._url_parts.set_path(format_args!("/api/v1/technologies"))?;
    }
  }

  technologies() {
    || {
      TechnologiesReq
    }
  }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Debug)]
pub struct TechnologiesRes {
  pub technologies: Vec<TechnologyRes>,
}
