use crate::gaming::age_of_empires_ii::{AgeOfEmpiresII, StructureRes};
use alloc::vec::Vec;
use lucia::{
  data_formats::{JsonRequest, JsonResponse},
  network::http::Method,
};

_create_endpoint! {
  AgeOfEmpiresII => JsonResponse|JsonRequest|json_request;

  StructuresReq<;;>

  |raw: StructuresRes, _resp| -> StructuresRes { Ok(raw) }

  StructuresParams() -> crate::Result<()> {
    |hp| {
      hp.tp.method = Method::Get;
      hp.tp.url_parts.set_path(format_args!("/api/v1/structures"))?;
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
