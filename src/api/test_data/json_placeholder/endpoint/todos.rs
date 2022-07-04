use crate::{api::test_data::json_placeholder::JsonPlaceholder, network::HttpMethod};
use alloc::vec::Vec;
use arrayvec::ArrayString;

_create_json_endpoint! {
  JsonPlaceholder;

  TodosReq<;;>

  |raw: Vec<TodosRes>| -> Vec<TodosRes> { Ok(raw) }

  todos() -> crate::Result<:> {
    |api, tp| {
      tp.http_params._set(HttpMethod::Get, &api.origin);
      tp.http_params.url.try_push_str("/todos")?;
      TodosReq
    }
  }

  Ok
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TodosRes {
  pub user_id: u32,
  pub id: u32,
  pub title: ArrayString<86>,
  pub completed: bool,
}
