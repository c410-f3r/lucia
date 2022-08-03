use crate::art_and_design::colour_lovers::{
  ColorsParams, CommonReqTy, LoversParams, LoversTy, PalettesParams, PatternsParams, StatsParams,
  StatsTy,
};
use lucia::{
  dnsn::SerdeXmlRs,
  misc::CommonParams,
  network::{http::ReqParams, Transport},
};

_create_http_test!(http(), colors, |rmw, trans| async {
  let req = rmw.colors();
  let _ = trans
    .send_retrieve_and_decode_one(
      rmw,
      &req,
      ColorsParams::new(CommonReqTy::All, None, None, None, None),
    )
    .await
    .unwrap();
});

_create_http_test!(http(), lovers, |rmw, trans| async {
  let req = rmw.lovers();
  let _ = trans
    .send_retrieve_and_decode_one(rmw, &req, LoversParams::new(LoversTy::All, None))
    .await
    .unwrap();
});

_create_http_test!(http(), palettes, |rmw, trans| async {
  let req = rmw.palettes();
  let _ = trans
    .send_retrieve_and_decode_one(
      rmw,
      &req,
      PalettesParams::new(CommonReqTy::All, None, None, None, None, None),
    )
    .await
    .unwrap();
});

_create_http_test!(http(), patterns, |rmw, trans| async {
  let req = rmw.patterns();
  let _ = trans
    .send_retrieve_and_decode_one(
      rmw,
      &req,
      PatternsParams::new(CommonReqTy::All, None, None, None, None, None),
    )
    .await
    .unwrap();
});

_create_http_test!(http(), stats, |rmw, trans| async {
  let req = rmw.stats();
  let _ =
    trans.send_retrieve_and_decode_one(rmw, &req, StatsParams::new(StatsTy::Colors)).await.unwrap();
});

fn http() -> (CommonParams<ReqParams, ()>, SerdeXmlRs) {
  (
    CommonParams::new(ReqParams::from_origin("http://www.colourlovers.com").unwrap(), ()),
    SerdeXmlRs::default(),
  )
}
