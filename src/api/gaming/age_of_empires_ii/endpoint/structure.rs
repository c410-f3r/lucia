use crate::{
  api::gaming::age_of_empires_ii::{AgeOfEmpiresII, CostRes},
  data_format::{JsonRequest, JsonResponse},
  network::http::Method,
};
use alloc::{boxed::Box, vec::Vec};
use arrayvec::ArrayString;

type Res = Box<StructureRes>;

_create_endpoint! {
  AgeOfEmpiresII => JsonResponse|JsonRequest|_json_request;

  StructureReq<;;>

  |raw: Res, _resp| -> Res { Ok(raw) }

  StructureParams(id: u64) -> crate::Result<()> {
    |hp| {
      hp.tp._method = Method::Get;
      hp.tp._url_parts.set_path(format_args!("/api/v1/structure/{id}"))?;
    }
  }

  structure() {
    || {
      StructureReq
    }
  }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Debug)]
pub struct StructureRes {
  pub id: u32,
  pub name: ArrayString<21>,
  pub description: Option<ArrayString<201>>,
  pub expansion: ArrayString<14>,
  pub age: ArrayString<8>,
  pub cost: CostRes,
  pub build_time: Option<u16>,
  pub hit_points: u16,
  pub line_of_sight: Option<u8>,
  pub armor: ArrayString<5>,
  pub reload_time: Option<f32>,
  pub attack: Option<u8>,
  pub special: Option<Vec<ArrayString<83>>>,
}
