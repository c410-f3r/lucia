use crate::{
  api::gaming::age_of_empires_ii::{AgeOfEmpiresII, CostRes},
  data_format::{JsonRequest, JsonResponse},
  network::http::Method,
};
use alloc::{boxed::Box, vec::Vec};
use arrayvec::ArrayString;

type Res = Box<TechnologyRes>;

_create_endpoint! {
  AgeOfEmpiresII => JsonResponse|JsonRequest|_json_request;

  TechnologyReq<;;>

  |raw: Res, _resp| -> Res { Ok(raw) }

  TechnologyParams(id: u64) -> crate::Result<()> {
    |hp| {
      hp.tp._method = Method::Get;
      hp.tp._url_parts.set_path(format_args!("/api/v1/technology/{id}"))?;
    }
  }

  technology() {
    || {
      TechnologyReq
    }
  }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Debug)]
pub struct TechnologyRes {
  pub id: u32,
  pub name: ArrayString<21>,
  pub description: ArrayString<201>,
  pub expansion: ArrayString<14>,
  pub age: ArrayString<8>,
  pub develops_in: ArrayString<74>,
  pub cost: CostRes,
  pub build_time: Option<u8>,
  pub applies_to: Option<Vec<ArrayString<73>>>,
}
