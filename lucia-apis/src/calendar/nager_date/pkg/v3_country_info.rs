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
    params: &mut V3CountryInfoParams<'_>,
    req_params: &mut HttpReqParams,
  ) -> crate::Result<()> {
    req_params.url.push_path(format_args!("/api/v3/CountryInfo/{}", params.country))?;
    Ok(())
  }

  #[derive(Debug)]
  #[pkg::params]
  pub struct V3CountryInfoParams<'any> {
    country: &'any str,
  }

  #[cfg_attr(feature = "serde", derive(serde::Serialize))]
  #[derive(Debug)]
  #[pkg::req_data]
  pub struct V3CountryInfoReq;

  #[pkg::res_data]
  pub type V3CountryInfoRes = Box<V3CountryInfoResElem>;

  #[cfg_attr(feature = "serde", derive(serde::Deserialize))]
  #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
  #[derive(Debug)]
  #[doc = _generic_res_data_elem_doc!()]
  pub struct V3CountryInfoResElem {
    /// For example, Spain.
    pub common_name: ArrayString<12>,
    /// For example, Kingdom of Spain.
    pub official_name: ArrayString<26>,
    /// ISO 3166-1 alpha-2.
    pub country_code: ArrayString<12>,
    /// Continent.
    pub region: ArrayString<6>,
    /// Adjacent countries.
    pub borders: Option<Vec<V3CountryInfoResElem>>,
  }
}
