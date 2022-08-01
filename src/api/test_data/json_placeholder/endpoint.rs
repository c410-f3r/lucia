mod albums;
mod comments;
mod photos;
mod posts;
mod todos;
mod users;

pub use albums::*;
pub use comments::*;
pub use photos::*;
pub use posts::*;
pub use todos::*;
pub use users::*;

use crate::{
  network::http::{Method, ReqParams},
  utils::DebugDisplay,
  CommonParams,
};
use alloc::{boxed::Box, vec::Vec};

pub(crate) type ResBox = Box<Res>;

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[derive(Debug)]
pub enum Res {
  Album(AlbumRes),
  Albums(Vec<AlbumRes>),
  Comment(CommentRes),
  Comments(Vec<CommentRes>),
  Photo(PhotoRes),
  Photos(Vec<PhotoRes>),
  Post(PostRes),
  Posts(Vec<PostRes>),
  Todo(TodoRes),
  Todos(Vec<TodoRes>),
  User(UserRes),
  Users(Vec<UserRes>),
}

fn params_management<'reqp, UP>(
  endpoint: &str,
  cp: &mut CommonParams<ReqParams, UP>,
  method: Method,
  id_opt: Option<u32>,
  nested_opt: Option<&'reqp str>,
  query: &'reqp [(&'reqp str, &'reqp dyn DebugDisplay)],
) -> crate::Result<()> {
  cp.tp._method = method;
  match (id_opt, nested_opt) {
    (None, None) | (None, Some(_)) => cp.tp._url_parts.set_path(format_args!("/{endpoint}"))?,
    (Some(id), None) => cp.tp._url_parts.set_path(format_args!("/{endpoint}/{id}"))?,
    (Some(id), Some(nested)) => {
      cp.tp._url_parts.set_path(format_args!("/{endpoint}/{id}/{nested}"))?
    }
  }
  let mut query_writer = cp.tp._url_parts.query_writer();
  for (key, value) in query {
    query_writer = query_writer.write(key, value)?;
  }
  Ok(())
}
