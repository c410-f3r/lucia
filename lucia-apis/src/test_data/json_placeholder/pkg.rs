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

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[derive(Debug)]
#[lucia_macros::pkg_doc]
pub enum GenericResData {
  Album(AlbumsElemResData),
  Albums(Vec<AlbumsElemResData>),
  Comment(CommentsElemResData),
  Comments(Vec<CommentsElemResData>),
  Photo(Box<PhotosElemResData>),
  Photos(Vec<PhotosElemResData>),
  Post(PostsElemResData),
  Posts(Vec<PostsElemResData>),
  Todo(TodosElemResData),
  Todos(Vec<TodosElemResData>),
  User(UsersElemResData),
  Users(Vec<UsersElemResData>),
}

#[derive(Debug)]
#[lucia_macros::pkg_doc]
pub struct GenericParams<'any> {
  pub id_opt: Option<u32>,
  pub method: HttpMethod,
  pub nested_opt: Option<&'any str>,
  pub query: &'any [(&'any str, &'any (dyn DebugDisplay + Sync))],
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
