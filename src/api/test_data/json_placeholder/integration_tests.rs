use crate::{
  api::{test_data::json_placeholder::JsonPlaceholder, Api},
  network::Transport,
};

_create_http_test!(http(), albums, |rb, trans| async {
  let req = rb.albums().unwrap();
  let _ = trans.send_retrieve_and_decode_one(&req, rb.tp_mut()).await.unwrap();
});

_create_http_test!(http(), comments, |rb, trans| async {
  let req = rb.comments().unwrap();
  let _ = trans.send_retrieve_and_decode_one(&req, rb.tp_mut()).await.unwrap();
});

_create_http_test!(http(), photos, |rb, trans| async {
  let req = rb.photos().unwrap();
  let _ = trans.send_retrieve_and_decode_one(&req, rb.tp_mut()).await.unwrap();
});

_create_http_test!(http(), posts, |rb, trans| async {
  let req = rb.posts().unwrap();
  let _ = trans.send_retrieve_and_decode_one(&req, rb.tp_mut()).await.unwrap();
});

_create_http_test!(http(), todos, |rb, trans| async {
  let req = rb.todos().unwrap();
  let _ = trans.send_retrieve_and_decode_one(&req, rb.tp_mut()).await.unwrap();
});

_create_http_test!(http(), users, |rb, trans| async {
  let req = rb.users().unwrap();
  let _ = trans.send_retrieve_and_decode_one(&req, rb.tp_mut()).await.unwrap();
});

fn http() -> JsonPlaceholder {
  JsonPlaceholder::new("https://jsonplaceholder.typicode.com", ()).unwrap()
}
