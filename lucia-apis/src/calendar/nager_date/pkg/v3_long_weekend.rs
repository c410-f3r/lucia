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
  async fn before_sending(
    params: &mut V3LongWeekendParams<'_>,
    req_params: &mut HttpReqParams,
  ) -> crate::Result<()> {
    req_params
      .url
      .push_path(format_args!("/api/v3/LongWeekend/{}/{}", params.year, params.country_code))?;
    Ok(())
  }

  #[derive(Debug)]
  #[pkg::params]
  pub struct V3LongWeekendParams<'any> {
    year: i16,
    country_code: &'any str,
  }

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[pkg::req_data]
  pub struct V3LongWeekendReq;

  #[pkg::res_data]
  pub type V3LongWeekendRes = Vec<V3LongWeekendResElem>;

  #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
  #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
  #[derive(Debug)]
  #[doc = _generic_res_data_elem_doc!()]
  pub struct V3LongWeekendResElem {
    /// Start date.
    pub start_date: ArrayString<10>,
    /// End date.
    pub end_date: ArrayString<10>,
    /// Number os days.
    pub day_count: u8,
    /// A working day that is sandwiched between a holiday and a weekend.
    pub need_bridge_day: bool,
  }
}
