use crate::{
  api::art_and_design::colour_lovers::{
    ColourLovers, CommonReqTy, ContentParams, FilterParams, DATE_LEN, TITLE_LEN, URL_LEN,
    USER_NAME_LEN,
  },
  data_format::{XmlRequest, XmlResponse},
  network::http::{Method, UserAgent},
};
use alloc::{string::String, vec::Vec};
use arrayvec::ArrayString;

type Res = Vec<ColorsRes>;

_create_endpoint! {
  ColourLovers => XmlResponse|XmlRequest|_xml_request;

  ColorsReq<;;>

  |raw: Res, _resp| -> Res { Ok(raw) }

  ColorsParams(
    ty: CommonReqTy,
    hue_range: Option<&'reqp str>,
    bri_range: Option<&'reqp str>,
    cp: Option<ContentParams<'reqp>>,
    fp: Option<FilterParams<'reqp>>
  ) -> crate::Result<()> {
    |_hp| {
      _hp.tp._method = Method::Get;
      _hp.tp._user_agent = Some(UserAgent::_Mozilla);
      match ty {
        CommonReqTy::All => _hp.tp._url_parts.set_path(format_args!("/api/colors"))?,
        CommonReqTy::New => _hp.tp._url_parts.set_path(format_args!("/api/colors/new"))?,
        CommonReqTy::Random => _hp.tp._url_parts.set_path(format_args!("/api/colors/random"))?,
        CommonReqTy::Top => _hp.tp._url_parts.set_path(format_args!("/api/colors/top"))?,
      }
      let mut qw = _hp.tp._url_parts.query_writer()
        .write_opt("hueRange", hue_range)?
        .write_opt("briRange", bri_range)?;
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

  colors() {
    || {
      ColorsReq
    }
  }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename = "colors"))]
#[derive(Debug)]
pub struct ColorsRes {
  #[cfg_attr(feature = "serde", serde(rename = "color"))]
  pub colors: Vec<ColorRes>,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct ColorRes {
  pub id: u32,
  pub title: ArrayString<TITLE_LEN>,
  pub user_name: ArrayString<USER_NAME_LEN>,
  pub num_views: u32,
  pub num_votes: u32,
  pub num_comments: u32,
  pub num_hearts: f32,
  pub rank: u32,
  pub date_created: ArrayString<DATE_LEN>,
  pub hex: ArrayString<7>,
  pub rgb: ColorsRgbRes,
  pub hsv: ColorsHsvRes,
  pub description: String,
  pub url: ArrayString<URL_LEN>,
  pub image_url: ArrayString<URL_LEN>,
  pub badge_url: ArrayString<URL_LEN>,
  pub api_url: ArrayString<URL_LEN>,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Debug)]
pub struct ColorsRgbRes {
  pub red: u8,
  pub green: u8,
  pub blue: u8,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Debug)]
pub struct ColorsHsvRes {
  pub hue: u16,
  pub saturation: u8,
  pub value: u8,
}
