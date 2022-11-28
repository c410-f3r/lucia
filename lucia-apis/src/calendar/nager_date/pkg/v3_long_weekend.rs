#[lucia_macros::pkg(
  api(crate::calendar::nager_date::NagerDate),
  data_format(json),
  error(crate::Error),
  transport(http)
)]
pub(crate) mod pkg {
  use crate::calendar::nager_date::NagerDateHttpPackagesAux;
  use alloc::vec::Vec;
  use arrayvec::ArrayString;
  use lucia::network::HttpReqParams;

  #[pkg::aux]
  impl<DRSR> NagerDateHttpPackagesAux<DRSR> {}

  #[pkg::before_sending]
  fn before_sending(
    params: &mut V3LongWeekendParams<'_>,
    req_params: &mut HttpReqParams,
  ) -> crate::Result<()> {
    req_params
      .url
      .push_path(format_args!("/api/v3/LongWeekend/{}/{}", params.year, params.country_code))?;
    Ok(())
  }

  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  #[pkg::params]
  pub struct V3LongWeekendParams<'any> {
    year: i16,
    country_code: &'any str,
  }

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  #[pkg::req_data]
  pub struct V3LongWeekendReqData;

  #[lucia_macros::pkg_doc]
  #[pkg::res_data]
  pub type V3LongWeekendResData = Vec<V3LongWeekendElemResData>;

  #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
  #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
  #[derive(Debug)]
  #[lucia_macros::pkg_doc]
  pub struct V3LongWeekendElemResData {
    pub start_date: ArrayString<10>,
    pub end_date: ArrayString<10>,
    pub day_count: u8,
    pub need_bridge_day: bool,
  }
}
