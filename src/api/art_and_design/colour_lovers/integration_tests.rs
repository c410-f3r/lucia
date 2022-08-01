use crate::{
  api::art_and_design::colour_lovers::{
    ColorsParams, CommonReqTy, LoversParams, LoversTy, PalettesParams, PatternsParams, StatsParams,
    StatsTy,
  },
  dnsn::SerdeXmlRs,
  network::{http::ReqParams, Transport},
  CommonParams,
};

_create_http_test!(http(), colors, |rm, trans| async {
  let req = rm.colors();
  let _ = trans
    .send_retrieve_and_decode_one(
      rm,
      &req,
      ColorsParams::new(CommonReqTy::All, None, None, None, None),
    )
    .await
    .unwrap();
});

_create_http_test!(http(), lovers, |rm, trans| async {
  let req = rm.lovers();
  let _ = trans
    .send_retrieve_and_decode_one(rm, &req, LoversParams::new(LoversTy::All, None))
    .await
    .unwrap();
});

_create_http_test!(http(), palettes, |rm, trans| async {
  let req = rm.palettes();
  let _ = trans
    .send_retrieve_and_decode_one(
      rm,
      &req,
      PalettesParams::new(CommonReqTy::All, None, None, None, None, None),
    )
    .await
    .unwrap();
});

_create_http_test!(http(), patterns, |rm, trans| async {
  let req = rm.patterns();
  let _ = trans
    .send_retrieve_and_decode_one(
      rm,
      &req,
      PatternsParams::new(CommonReqTy::All, None, None, None, None, None),
    )
    .await
    .unwrap();
});

_create_http_test!(http(), stats, |rm, trans| async {
  let req = rm.stats();
  let _ =
    trans.send_retrieve_and_decode_one(rm, &req, StatsParams::new(StatsTy::Colors)).await.unwrap();
});

fn http() -> (CommonParams<ReqParams, ()>, SerdeXmlRs) {
  (
    CommonParams::new(ReqParams::from_origin("http://www.colourlovers.com").unwrap(), ()),
    SerdeXmlRs::default(),
  )
}
