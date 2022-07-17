#![cfg(all(test, feature = "_integration-tests"))]

use crate::{
  api::test_data::json_placeholder::{
    AlbumsParams, CommentsParams, PhotosParams, PostsParams, TodosParams, UsersParams,
  },
  network::{HttpParams, Transport},
};

_create_http_test!(http(), albums, |rm, trans| async {
  let req = rm.albums();
  let _ = trans.send_retrieve_and_decode_one(rm, &req, AlbumsParams::new()).await.unwrap();
});

_create_http_test!(http(), comments, |rm, trans| async {
  let req = rm.comments();
  let _ = trans.send_retrieve_and_decode_one(rm, &req, CommentsParams::new()).await.unwrap();
});

_create_http_test!(http(), photos, |rm, trans| async {
  let req = rm.photos();
  let _ = trans.send_retrieve_and_decode_one(rm, &req, PhotosParams::new()).await.unwrap();
});

_create_http_test!(http(), posts, |rm, trans| async {
  let req = rm.posts();
  let _ = trans.send_retrieve_and_decode_one(rm, &req, PostsParams::new()).await.unwrap();
});

_create_http_test!(http(), todos, |rm, trans| async {
  let req = rm.todos();
  let _ = trans.send_retrieve_and_decode_one(rm, &req, TodosParams::new()).await.unwrap();
});

_create_http_test!(http(), users, |rm, trans| async {
  let req = rm.users();
  let _ = trans.send_retrieve_and_decode_one(rm, &req, UsersParams::new()).await.unwrap();
});

fn http() -> HttpParams {
  HttpParams::from_origin("https://jsonplaceholder.typicode.com").unwrap()
}
