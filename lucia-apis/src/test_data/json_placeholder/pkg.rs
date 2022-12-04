mod albums;
mod comments;
mod photos;
mod posts;
mod todos;
mod users;

pub use albums::pkg::*;
pub use comments::pkg::*;
pub use photos::pkg::*;
pub use posts::pkg::*;
pub use todos::pkg::*;
pub use users::pkg::*;

use alloc::{boxed::Box, vec::Vec};
use lucia::{
  misc::DebugDisplay,
  network::{HttpMethod, HttpReqParams},
};

/// Generic response used by all packages.
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[derive(Debug)]
pub enum GenericRes {
  /// One album.
  Album(Box<Album>),
  /// Multiple albums.
  Albums(Vec<Album>),
  /// One comment.
  Comment(Box<Comment>),
  /// Multiple comments.
  Comments(Vec<Comment>),
  /// One photo.
  Photo(Box<Photo>),
  /// Multiple photos.
  Photos(Vec<Photo>),
  /// One post.
  Post(Box<Post>),
  /// Multiple posts.
  Posts(Vec<Post>),
  /// One todo.
  Todo(Box<Todo>),
  /// Multiple todos.
  Todos(Vec<Todo>),
  /// One user.
  User(Box<User>),
  /// Multiple users.
  Users(Vec<User>),
}

/// Generic parameters used by all packages.
#[derive(Debug)]
pub struct GenericParams<'any> {
  id_opt: Option<u32>,
  method: HttpMethod,
  nested_opt: Option<&'any str>,
  query: &'any [(&'any str, &'any (dyn DebugDisplay + Sync))],
}

impl<'any> GenericParams<'any> {
  /// Constructor shortcut
  pub const fn new(
    id_opt: Option<u32>,
    method: HttpMethod,
    nested_opt: Option<&'any str>,
    query: &'any [(&'any str, &'any (dyn DebugDisplay + Sync))],
  ) -> Self {
    Self { id_opt, method, nested_opt, query }
  }
}

fn params_management(
  endpoint: &str,
  params: &mut GenericParams<'_>,
  req_params: &mut HttpReqParams,
) -> crate::Result<()> {
  req_params.method = params.method;
  match (params.id_opt, params.nested_opt) {
    (None, None) | (None, Some(_)) => req_params.url.push_path(format_args!("/{endpoint}"))?,
    (Some(id), None) => req_params.url.push_path(format_args!("/{endpoint}/{id}"))?,
    (Some(id), Some(nested)) => {
      req_params.url.push_path(format_args!("/{endpoint}/{id}/{nested}"))?
    }
  }
  let mut query_writer = req_params.url.query_writer()?;
  for (key, value) in params.query {
    query_writer = query_writer.write(key, value)?;
  }
  Ok(())
}
