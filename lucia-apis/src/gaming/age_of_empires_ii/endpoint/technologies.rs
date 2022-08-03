use crate::gaming::age_of_empires_ii::{AgeOfEmpiresII, TechnologyRes};
use alloc::vec::Vec;
use lucia::{
  data_formats::{JsonRequest, JsonResponse},
  network::http::Method,
};

_create_endpoint! {
  AgeOfEmpiresII => JsonResponse|JsonRequest|json_request;

  TechnologiesReq<;;>

  |raw: TechnologiesRes, _resp| -> TechnologiesRes { Ok(raw) }

  TechnologiesParams() -> crate::Result<()> {
    |hp| {
      hp.tp.method = Method::Get;
      hp.tp.url_parts.set_path(format_args!("/api/v1/technologies"))?;
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
