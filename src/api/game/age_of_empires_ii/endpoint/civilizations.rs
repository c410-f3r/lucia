use crate::{
  api::game::age_of_empires_ii::{endpoint::CivilizationRes, AgeOfEmpiresII},
  network::HttpMethod,
};
use alloc::vec::Vec;

type Res = CivilizationsRes;

_create_json_endpoint! {
  AgeOfEmpiresII;

  CivilizationsReq<;;>

  |raw: Res| -> Res { Ok(raw) }

  civilizations() {
    |api, tp| {
      tp.http_params._set(HttpMethod::Get, &api.origin);
      let _rslt = tp.http_params.url.try_push_str("/api/v1/civilizations");
      CivilizationsReq
    }
  }
}

#[derive(Debug, serde::Deserialize)]
pub struct CivilizationsRes {
  pub civilizations: Vec<CivilizationRes>,
}