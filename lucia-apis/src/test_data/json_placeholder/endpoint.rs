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

use alloc::{boxed::Box, vec::Vec};
use lucia::{
  misc::{CommonParams, DebugDisplay},
  network::http::{Method, ReqParams},
};

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
  cp.tp.method = method;
  match (id_opt, nested_opt) {
    (None, None) | (None, Some(_)) => cp.tp.url_parts.set_path(format_args!("/{endpoint}"))?,
    (Some(id), None) => cp.tp.url_parts.set_path(format_args!("/{endpoint}/{id}"))?,
    (Some(id), Some(nested)) => {
      cp.tp.url_parts.set_path(format_args!("/{endpoint}/{id}/{nested}"))?
    }
  }
  let mut query_writer = cp.tp.url_parts.query_writer();
  for (key, value) in query {
    query_writer = query_writer.write(key, value)?;
  }
  Ok(())
}
