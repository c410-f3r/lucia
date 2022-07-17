use crate::{api::test_data::json_placeholder::JsonPlaceholder, network::HttpMethod};
use alloc::vec::Vec;
use arrayvec::ArrayString;

_create_json_endpoint! {
  JsonPlaceholder;

  TodosReq<;;>

  |raw: Vec<TodosRes>| -> Vec<TodosRes> { Ok(raw) }

  TodosParams() -> crate::Result<()> {
    |cp| {
      cp._method = HttpMethod::_Get;
      cp._url_parts.set_path(format_args!("/todos"))?;
    }
  }

  todos() {
    || {
      TodosReq
    }
  }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TodosRes {
  pub user_id: u32,
  pub id: u32,
  pub title: ArrayString<86>,
  pub completed: bool,
}
