use crate::test_data::json_placeholder::{endpoint::params_management, JsonPlaceholder, ResBox};
use arrayvec::ArrayString;
use lucia::{
  data_formats::{JsonRequest, JsonResponse},
  misc::DebugDisplay,
  network::http::Method,
};

_create_endpoint! {
  JsonPlaceholder => JsonResponse|JsonRequest|json_request;

  TodosReq<;;>

  |raw: ResBox, _resp| -> ResBox { Ok(raw) }

  TodosParams(
    method: Method,
    id_opt: Option<u32>,
    nested_opt: Option<&'reqp str>,
    query: &'reqp [(&'reqp str, &'reqp dyn DebugDisplay)]
  ) -> crate::Result<()> {
    |cp| {
      params_management("todos", cp, method, id_opt, nested_opt, query)?;
    }
  }

  todos() {
    || {
      TodosReq
    }
  }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct TodoRes {
  pub user_id: u32,
  pub id: u32,
  pub title: ArrayString<86>,
  pub completed: bool,
}
