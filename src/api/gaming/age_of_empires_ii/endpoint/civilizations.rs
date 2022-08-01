use crate::{
  api::gaming::age_of_empires_ii::{AgeOfEmpiresII, CivilizationRes},
  data_format::{JsonRequest, JsonResponse},
  network::http::Method,
};
use alloc::vec::Vec;

type Res = CivilizationsRes;

_create_endpoint! {
  AgeOfEmpiresII => JsonResponse|JsonRequest|_json_request;

  CivilizationsReq<;;>

  |raw: Res, _resp| -> Res { Ok(raw) }

  CivilizationsParams() -> crate::Result<()> {
    |hp| {
      hp.tp._method = Method::Get;
      hp.tp._url_parts.set_path(format_args!("/api/v1/civilizations"))?;
    }
  }

  civilizations() {
    || {
      CivilizationsReq
    }
  }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Debug)]
pub struct CivilizationsRes {
  pub civilizations: Vec<CivilizationRes>,
}
