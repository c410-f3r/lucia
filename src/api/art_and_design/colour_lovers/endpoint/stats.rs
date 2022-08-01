use crate::{
  api::art_and_design::colour_lovers::ColourLovers,
  data_format::{XmlRequest, XmlResponse},
  network::http::{Method, UserAgent},
};

type Res = StatsRes;

_create_endpoint! {
  ColourLovers => XmlResponse|XmlRequest|_xml_request;

  StatsReq<;;>

  |raw: Res, _resp| -> Res { Ok(raw) }

  StatsParams(
    ty: StatsTy,
  ) -> crate::Result<()> {
    |_hp| {
      _hp.tp._method = Method::Get;
      _hp.tp._user_agent = Some(UserAgent::_Mozilla);
      match ty {
        StatsTy::Colors => _hp.tp._url_parts.set_path(format_args!("/api/stats/colors"))?,
        StatsTy::Lovers => _hp.tp._url_parts.set_path(format_args!("/api/stats/lovers"))?,
        StatsTy::Palettes => _hp.tp._url_parts.set_path(format_args!("/api/stats/palettes"))?,
        StatsTy::Patterns => _hp.tp._url_parts.set_path(format_args!("/api/stats/patterns"))?,
      }
    }
  }

  stats() {
    || {
      StatsReq
    }
  }
}

#[derive(Debug)]
pub enum StatsTy {
  Colors,
  Lovers,
  Palettes,
  Patterns,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename = "stats"))]
#[derive(Debug)]
pub struct StatsRes {
  pub total: u32,
}
