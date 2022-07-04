use crate::{api::test_data::json_placeholder::JsonPlaceholder, network::Transport, Api};

_create_http_test!(api(), albums, |rb, trans| async {
  let req = rb.albums().unwrap();
  let _ = trans.send_retrieve_and_decode_one(&req, rb.tp_mut()).await.unwrap();
});

_create_http_test!(api(), comments, |rb, trans| async {
  let req = rb.comments().unwrap();
  let _ = trans.send_retrieve_and_decode_one(&req, rb.tp_mut()).await.unwrap();
});

_create_http_test!(api(), photos, |rb, trans| async {
  let req = rb.photos().unwrap();
  let _ = trans.send_retrieve_and_decode_one(&req, rb.tp_mut()).await.unwrap();
});

_create_http_test!(api(), posts, |rb, trans| async {
  let req = rb.posts().unwrap();
  let _ = trans.send_retrieve_and_decode_one(&req, rb.tp_mut()).await.unwrap();
});

_create_http_test!(api(), todos, |rb, trans| async {
  let req = rb.todos().unwrap();
  let _ = trans.send_retrieve_and_decode_one(&req, rb.tp_mut()).await.unwrap();
});

_create_http_test!(api(), users, |rb, trans| async {
  let req = rb.users().unwrap();
  let _ = trans.send_retrieve_and_decode_one(&req, rb.tp_mut()).await.unwrap();
});

fn api() -> JsonPlaceholder {
  JsonPlaceholder::from_origin("https://jsonplaceholder.typicode.com").unwrap()
}
