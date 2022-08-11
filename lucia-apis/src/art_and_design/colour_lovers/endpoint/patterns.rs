use crate::art_and_design::colour_lovers::{
  Colors, ColourLovers, CommonReqTy, ContentParams, FilterParams, DATE_LEN, TITLE_LEN, URL_LEN,
  USER_NAME_LEN,
};
use alloc::{string::String, vec::Vec};
use arrayvec::{ArrayString, ArrayVec};
use lucia::{
  data_formats::{XmlRequest, XmlResponse},
  network::http::{Method, UserAgent},
};

type Res = Vec<PatternsRes>;

_create_endpoint! {
  ColourLovers => XmlResponse|XmlRequest|xml_request;

  PatternsReq<;;>

  |raw: Res, _resp| -> Res { Ok(raw) }

  PatternsParams(
    ty: CommonReqTy,
    hue_option: Option<&'reqp str>,
    hex: Option<&'reqp str>,
    hex_logic: Option<&'reqp str>,
    cp: Option<ContentParams<'reqp>>,
    fp: Option<FilterParams<'reqp>>
  ) -> crate::Result<()> {
    |_hp| {
      _hp.tp.method = Method::Get;
      _hp.tp.user_agent = Some(UserAgent::Mozilla);
      match ty {
        CommonReqTy::All => _hp.tp.url_parts.set_path(format_args!("/api/patterns"))?,
        CommonReqTy::New => _hp.tp.url_parts.set_path(format_args!("/api/patterns/new"))?,
        CommonReqTy::Random => _hp.tp.url_parts.set_path(format_args!("/api/patterns/random"))?,
        CommonReqTy::Top => _hp.tp.url_parts.set_path(format_args!("/api/patterns/top"))?,
      }
      let mut qw = _hp.tp.url_parts.query_writer()
        .write_opt("hueOption", hue_option)?
        .write_opt("hex", hex)?
        .write_opt("hexLogic", hex_logic)?;
      if let Some(ContentParams { lover, keywords, keyword_exact }) = cp {
        qw = qw.write_opt("lover", lover)?
          .write_opt("keywords", keywords)?
          .write_opt("keywordExact", keyword_exact)?;
      }
      if let Some(FilterParams { order_col, sort_by, num_result, result_offset }) = fp {
        let _ = qw.write_opt("orderCol", order_col)?
        .write_opt("sortBy", sort_by)?
        .write_opt("numResult", num_result)?
        .write_opt("resultOffset", result_offset)?;
      }
    }
  }

  patterns() {
    || {
      PatternsReq
    }
  }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename = "patterns"))]
#[derive(Debug)]
pub struct PatternsRes {
  #[cfg_attr(feature = "serde", serde(rename = "pattern"))]
  pub patterns: Vec<PatternRes>,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct PatternRes {
  pub id: u32,
  pub title: ArrayString<TITLE_LEN>,
  pub user_name: ArrayString<USER_NAME_LEN>,
  pub num_views: u32,
  pub num_votes: u32,
  pub num_comments: u32,
  pub num_hearts: f32,
  pub rank: u32,
  pub date_created: ArrayString<DATE_LEN>,
  pub colors: Colors,
  pub description: String,
  pub url: ArrayString<URL_LEN>,
  pub image_url: ArrayString<URL_LEN>,
  pub badge_url: ArrayString<URL_LEN>,
  pub api_url: ArrayString<URL_LEN>,
  pub color_widths: Option<ArrayVec<f32, 5>>,
}
