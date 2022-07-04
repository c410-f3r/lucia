use crate::{
  api::game::age_of_empires_ii::{endpoint::UnitRes, AgeOfEmpiresII},
  network::HttpMethod,
};
use alloc::vec::Vec;

type Res = UnitsRes;

_create_json_endpoint! {
  AgeOfEmpiresII;

  UnitsReq<;;>

  |raw: Res| -> Res { Ok(raw) }

  units() {
    |api, tp| {
      tp.http_params._set(HttpMethod::Get, &api.origin);
      let _rslt = tp.http_params.url.try_push_str("/api/v1/units");
      UnitsReq
    }
  }
}

#[derive(Debug, serde::Deserialize)]
pub struct UnitsRes {
  pub units: Vec<UnitRes>,
}
