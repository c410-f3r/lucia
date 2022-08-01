use crate::{
  api::gaming::age_of_empires_ii::{AgeOfEmpiresII, StructureRes},
  data_format::{JsonRequest, JsonResponse},
  network::http::Method,
};
use alloc::vec::Vec;

_create_endpoint! {
  AgeOfEmpiresII => JsonResponse|JsonRequest|_json_request;

  StructuresReq<;;>

  |raw: StructuresRes, _resp| -> StructuresRes { Ok(raw) }

  StructuresParams() -> crate::Result<()> {
    |hp| {
      hp.tp._method = Method::Get;
      hp.tp._url_parts.set_path(format_args!("/api/v1/structures"))?;
    }
  }

  structures() {
    || {
      StructuresReq
    }
  }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Debug)]
pub struct StructuresRes {
  pub structures: Vec<StructureRes>,
}
