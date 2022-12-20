use crate::{misc::init_test_cfg, series::rick_and_morty::RickAndMorty};
use lucia::{
  dnsn::SerdeJson,
  network::{transport::Transport, HttpParams},
};

create_http_test!(RickAndMorty, http(), character, |pkgs_aux, trans| async {
  let _res = trans
    .send_retrieve_and_decode_contained(
      &mut pkgs_aux.character().data(&mut String::new(), 1).unwrap().build(),
      pkgs_aux,
    )
    .await
    .unwrap()
    .data
    .result
    .unwrap();
});

create_http_test!(RickAndMorty, http(), characters, |pkgs_aux, trans| async {
  let _res = trans
    .send_retrieve_and_decode_contained(
      &mut pkgs_aux.characters().data(&mut String::new(), "", "", 0, "", "", "").unwrap().build(),
      pkgs_aux,
    )
    .await
    .unwrap()
    .data
    .result
    .unwrap();
});

create_http_test!(RickAndMorty, http(), characters_by_ids, |pkgs_aux, trans| async {
  let _res = trans
    .send_retrieve_and_decode_contained(
      &mut pkgs_aux.characters_by_ids().data(&mut String::new(), &[1, 2]).unwrap().build(),
      pkgs_aux,
    )
    .await
    .unwrap()
    .data
    .result
    .unwrap();
});

create_http_test!(RickAndMorty, http(), episode, |pkgs_aux, trans| async {
  let _res = trans
    .send_retrieve_and_decode_contained(
      &mut pkgs_aux.episode().data(&mut String::new(), 1).unwrap().build(),
      pkgs_aux,
    )
    .await
    .unwrap()
    .data
    .result
    .unwrap();
});

create_http_test!(RickAndMorty, http(), episodes, |pkgs_aux, trans| async {
  let _res = trans
    .send_retrieve_and_decode_contained(
      &mut pkgs_aux.episodes().data(&mut String::new(), "", "", 0).unwrap().build(),
      pkgs_aux,
    )
    .await
    .unwrap()
    .data
    .result
    .unwrap();
});

create_http_test!(RickAndMorty, http(), episodes_by_ids, |pkgs_aux, trans| async {
  let _res = trans
    .send_retrieve_and_decode_contained(
      &mut pkgs_aux.episodes_by_ids().data(&mut String::new(), &[1, 2]).unwrap().build(),
      pkgs_aux,
    )
    .await
    .unwrap()
    .data
    .result
    .unwrap();
});

create_http_test!(RickAndMorty, http(), location, |pkgs_aux, trans| async {
  let _res = trans
    .send_retrieve_and_decode_contained(
      &mut pkgs_aux.location().data(&mut String::new(), 1).unwrap().build(),
      pkgs_aux,
    )
    .await
    .unwrap()
    .data
    .result
    .unwrap();
});

create_http_test!(RickAndMorty, http(), locations, |pkgs_aux, trans| async {
  let _res = trans
    .send_retrieve_and_decode_contained(
      &mut pkgs_aux.locations().data(&mut String::new(), "", "", 0, "").unwrap().build(),
      pkgs_aux,
    )
    .await
    .unwrap()
    .data
    .result
    .unwrap();
});

create_http_test!(RickAndMorty, http(), locations_by_ids, |pkgs_aux, trans| async {
  let _res = trans
    .send_retrieve_and_decode_contained(
      &mut pkgs_aux.locations_by_ids().data(&mut String::new(), &[1, 2]).unwrap().build(),
      pkgs_aux,
    )
    .await
    .unwrap()
    .data
    .result
    .unwrap();
});

fn http() -> (SerdeJson, HttpParams) {
  (SerdeJson, HttpParams::from_url("https://rickandmortyapi.com/graphql").unwrap())
}
