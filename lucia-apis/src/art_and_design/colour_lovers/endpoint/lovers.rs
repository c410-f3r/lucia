use crate::art_and_design::colour_lovers::{
  ColourLovers, FilterParams, DATE_LEN, URL_LEN, USER_NAME_LEN,
};
use alloc::{string::String, vec::Vec};
use arrayvec::ArrayString;
use lucia::{
  data_formats::{XmlRequest, XmlResponse},
  network::http::{Method, UserAgent},
};

type Res = Vec<LoversRes>;

_create_endpoint! {
  ColourLovers => XmlResponse|XmlRequest|xml_request;

  LoversReq<;;>

  |raw: Res, _resp| -> Res { Ok(raw) }

  LoversParams(
    ty: LoversTy,
    fp: Option<FilterParams<'reqp>>
  ) -> crate::Result<()> {
    |_hp| {
      _hp.tp.method = Method::Get;
      _hp.tp.user_agent = Some(UserAgent::Mozilla);
      match ty {
        LoversTy::All => _hp.tp.url_parts.set_path(format_args!("/api/lovers"))?,
        LoversTy::New => _hp.tp.url_parts.set_path(format_args!("/api/lovers/new"))?,
        LoversTy::Top => _hp.tp.url_parts.set_path(format_args!("/api/lovers/top"))?,
      }
      if let Some(FilterParams { order_col, sort_by, num_result, result_offset }) = fp {
        let _ = _hp.tp.url_parts.query_writer()
          .write_opt("orderCol", order_col)?
          .write_opt("sortBy", sort_by)?
          .write_opt("numResult", num_result)?
          .write_opt("resultOffset", result_offset)?;
      }
    }
  }

  lovers() {
    || {
      LoversReq
    }
  }
}

#[derive(Debug)]
pub enum LoversTy {
  All,
  New,
  Top,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename = "comment"))]
#[derive(Debug)]
pub struct CommentRes {
  pub comment_date: ArrayString<URL_LEN>,
  pub comment_user_name: ArrayString<18>,
  pub comment_comments: String,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename = "lovers"))]
#[derive(Debug)]
pub struct LoversRes {
  #[cfg_attr(feature = "serde", serde(rename = "lover"))]
  pub lovers: Vec<LoverRes>,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct LoverRes {
  pub id: Option<u32>,
  pub user_name: ArrayString<USER_NAME_LEN>,
  pub date_registered: ArrayString<DATE_LEN>,
  pub date_last_active: ArrayString<DATE_LEN>,
  pub rating: u32,
  pub location: ArrayString<20>,
  pub num_colors: f32,
  pub num_palettes: f32,
  pub num_patterns: u16,
  pub num_comments_made: f32,
  pub num_lovers: u16,
  pub num_comments_on_profile: u16,
  pub comments: Option<Vec<CommentRes>>,
  pub url: ArrayString<URL_LEN>,
  pub api_url: ArrayString<URL_LEN>,
}
