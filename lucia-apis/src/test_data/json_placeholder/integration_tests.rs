use crate::test_data::json_placeholder::{
  AlbumsParams, CommentsParams, PhotosParams, PostsParams, TodosParams, UsersParams,
};
use lucia::{
  dnsn::SerdeJson,
  misc::CommonParams,
  network::{
    http::{Method, ReqParams},
    Transport,
  },
};

_create_http_test!(http(), albums, |rmw, trans| async {
  let req = rmw.albums();
  let _ = trans
    .send_retrieve_and_decode_one(rmw, &req, AlbumsParams::new(Method::Get, None, None, &[]))
    .await
    .unwrap();
});

_create_http_test!(http(), comments, |rmw, trans| async {
  let req = rmw.comments();
  let _ = trans
    .send_retrieve_and_decode_one(rmw, &req, CommentsParams::new(Method::Get, None, None, &[]))
    .await
    .unwrap();
});

_create_http_test!(http(), photos, |rmw, trans| async {
  let req = rmw.photos();
  let _ = trans
    .send_retrieve_and_decode_one(rmw, &req, PhotosParams::new(Method::Get, None, None, &[]))
    .await
    .unwrap();
});

_create_http_test!(http(), posts, |rmw, trans| async {
  let req = rmw.posts();
  let _ = trans
    .send_retrieve_and_decode_one(rmw, &req, PostsParams::new(Method::Get, None, None, &[]))
    .await
    .unwrap();
});

_create_http_test!(http(), todos, |rmw, trans| async {
  let req = rmw.todos();
  let _ = trans
    .send_retrieve_and_decode_one(rmw, &req, TodosParams::new(Method::Get, None, None, &[]))
    .await
    .unwrap();
});

_create_http_test!(http(), users, |rmw, trans| async {
  let req = rmw.users();
  let _ = trans
    .send_retrieve_and_decode_one(rmw, &req, UsersParams::new(Method::Get, None, None, &[]))
    .await
    .unwrap();
});

fn http() -> (CommonParams<ReqParams, ()>, SerdeJson) {
  (
    CommonParams::new(ReqParams::from_origin("https://jsonplaceholder.typicode.com").unwrap(), ()),
    SerdeJson::default(),
  )
}
