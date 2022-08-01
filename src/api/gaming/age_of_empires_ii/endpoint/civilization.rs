use crate::{
  api::gaming::age_of_empires_ii::AgeOfEmpiresII,
  data_format::{JsonRequest, JsonResponse},
  network::http::Method,
};
use alloc::{boxed::Box, vec::Vec};
use arrayvec::ArrayString;

type Res = Box<CivilizationRes>;

_create_endpoint! {
  AgeOfEmpiresII => JsonResponse|JsonRequest|_json_request;

  CivilizationReq<;;>

  |raw: Res, _resp| -> Res { Ok(raw) }

  CivilizationParams(id: u64) -> crate::Result<()> {
    |hp| {
      hp.tp._method = Method::Get;
      hp.tp._url_parts.set_path(format_args!("/api/v1/civilization/{id}"))?;
    }
  }

  civilization() {
    || {
      CivilizationReq
    }
  }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Debug)]
pub struct CivilizationRes {
  pub army_type: ArrayString<30>,
  pub civilization_bonus: Vec<ArrayString<118>>,
  pub expansion: ArrayString<17>,
  pub id: u32,
  pub name: ArrayString<10>,
  pub team_bonus: ArrayString<93>,
  pub unique_tech: Vec<ArrayString<74>>,
  pub unique_unit: Vec<ArrayString<70>>,
}
