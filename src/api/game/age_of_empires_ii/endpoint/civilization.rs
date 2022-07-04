use crate::{api::game::age_of_empires_ii::AgeOfEmpiresII, network::HttpMethod};
use alloc::vec::Vec;
use arrayvec::ArrayString;
use core::fmt::Write;

type Res = CivilizationRes;

_create_json_endpoint! {
  AgeOfEmpiresII;

  CivilizationReq<;;>

  |raw: Res| -> Res { Ok(raw) }

  civilization(id: u64) -> crate::Result<:> {
    |api, tp| {
      tp.http_params._set(HttpMethod::Get, &api.origin);
      tp.http_params.url.write_fmt(format_args!("/api/v1/civilization/{id}"))?;
      CivilizationReq
    }
  }

  Ok
}

#[derive(Debug, serde::Deserialize)]
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
